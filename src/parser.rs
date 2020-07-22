use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::{Err, IResult};

use crate::expr::{CombinedTerm, Constant, Query, Term};
use crate::util::first_char;

fn parse_const_int(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(s)
}

fn parse_const_str(s: &str) -> IResult<&str, &str> {
    delimited(tag("\""), is_not("\""), tag("\""))(s)
}

fn parse_const_name(s: &str) -> IResult<&str, &str> {
    if first_char(s).is_lowercase() {
        alphanumeric1(s)
    } else {
        Err(Err::Error(("var", ErrorKind::NoneOf)))
    }
}

fn parse_const(s: &str) -> IResult<&str, Constant> {
    if let Ok((i, o)) = parse_const_int(s) {
        Ok((i, Constant::Int(o)))
    } else if let Ok((i, o)) = parse_const_str(s) {
        Ok((i, Constant::Str(o.to_string())))
    } else if let Ok((i, o)) = parse_const_name(s) {
        Ok((i, Constant::Name(o.to_string())))
    } else {
        Err(Err::Error(("const", ErrorKind::NoneOf)))
    }
}

fn parse_var(s: &str) -> IResult<&str, &str> {
    if first_char(s).is_uppercase() {
        alpha1(s)
    } else {
        Err(Err::Error(("var", ErrorKind::NoneOf)))
    }
}

fn parse_term(s: &str) -> IResult<&str, Term> {
    if let Ok((i, o)) = parse_combined(s) {
        Ok((i, Term::Combined(o)))
    } else if let Ok((i, o)) = parse_const(s) {
        Ok((i, Term::Const(o)))
    } else if let Ok((i, o)) = parse_var(s) {
        Ok((i, Term::Var(o.to_string())))
    } else {
        Err(Err::Error(("term", ErrorKind::NoneOf)))
    }
}

fn parse_terms(s: &str) -> IResult<&str, Vec<Term>> {
    let mut v = Vec::new();
    let mut i = s;
    loop {
        match parse_term(i) {
            Ok((ni, term)) => {
                i = ni;
                v.push(term);
            }
            Err(e) => return Err(e),
        }
        match parse_comma(i) {
            Ok((ni, _)) => i = ni,
            Err(_) => return Ok((i, v)),
        }
    }
}

fn parse_comma(s: &str) -> IResult<&str, &str> {
    tag(",")(s)
}

fn parse_dot(s: &str) -> IResult<&str, &str> {
    tag(".")(s)
}

fn parse_args(s: &str) -> IResult<&str, Vec<Term>> {
    let mut i = s;
    let mut v = Vec::new();
    loop {
        match parse_term(i) {
            Ok((ni, term)) => {
                v.push(term);
                i = ni;
            }
            Err(e) => return Err(e),
        }
        match parse_comma(i) {
            Ok((ni, _)) => i = ni,
            Err(_) => return Ok((i, v)),
        }
    }
}

fn parse_paren(s: &str) -> IResult<&str, &str> {
    delimited(tag("("), is_not(")"), tag(")"))(s)
}

fn parse_functor(s: &str) -> IResult<&str, &str> {
    if first_char(s).is_lowercase() {
        alpha1(s)
    } else {
        Err(Err::Error(("functor", ErrorKind::NoneOf)))
    }
}

fn parse_combined(s: &str) -> IResult<&str, CombinedTerm> {
    match parse_functor(s) {
        Ok((i, functor)) => match parse_paren(i) {
            Ok((i, o)) => match parse_args(o) {
                Ok((_, args)) => Ok((
                    i,
                    CombinedTerm {
                        functor: functor.to_string(),
                        args,
                    },
                )),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn parse_file_name(input: &str) -> IResult<&str, &str> {
    delimited(tag("['"), is_not("']"), tag("']"))(input)
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    if let Ok((i, o)) = parse_file_name(input) {
        match parse_dot(i) {
            Ok(_) => Ok(("", Query::File(o.to_string()))),
            Err(e) => Err(e),
        }
    } else if let Ok((i, terms)) = parse_terms(input) {
        match parse_dot(i) {
            Ok(_) => Ok((i, Query::Terms(terms))),
            Err(e) => Err(e),
        }
    } else {
        Err(Err::Error(("parse_query", ErrorKind::NoneOf)))
    }
}
