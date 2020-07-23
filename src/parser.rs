use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::error::ErrorKind;
use nom::{delimited, do_parse, eof, is_not, many_till, map_res, named, tag};
use nom::{Err, IResult};

use crate::expr::{CombinedTerm, Constant, Expr, Query, Term};
use crate::util::first_char;

named!(parse_const_int<&str, i32>, map_res!(digit1, |s: &str| s.parse::<i32>()));
named!(parse_const_str<&str, &str>, delimited!(tag!("\""), is_not!("\""), tag!("\"")));

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

named!(parse_comma<&str, &str>, tag!(","));
named!(parse_dot<&str, &str>, tag!("."));

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

named!(parse_paren<&str, &str>, delimited!(tag!("("), is_not!(")"), tag!(")")));

fn parse_functor(s: &str) -> IResult<&str, &str> {
    if first_char(s).is_lowercase() {
        alpha1(s)
    } else {
        Err(Err::Error(("functor", ErrorKind::NoneOf)))
    }
}

named!(
    parse_combined<&str, CombinedTerm>,
    do_parse!(
        functor: parse_functor >>
        args: map_res!(parse_paren, parse_args) >>
        (CombinedTerm{functor: functor.to_string(), args: args.1})
    )
);

fn parse_combined_terms(s: &str) -> IResult<&str, Vec<CombinedTerm>> {
    let mut v = Vec::new();
    let mut i = s;
    loop {
        match parse_combined(i) {
            Ok((ni, c)) => {
                i = ni;
                v.push(c);
            }
            Err(e) => return Err(e),
        }
        match parse_comma(i) {
            Ok((ni, _)) => i = ni,
            Err(_) => return Ok((i, v)),
        }
    }
}

named!(
    parse_file_name<&str, &str>,
    delimited!(tag!("['"), is_not!("']"), tag!("']"))
);

pub fn parse_query(input: &str) -> Result<Query, Err<(&str, ErrorKind)>> {
    if let Ok((i, o)) = parse_file_name(input) {
        match parse_dot(i) {
            Ok(_) => Ok(Query::File(o.to_string())),
            Err(e) => Err(e),
        }
    } else if let Ok((i, terms)) = parse_terms(input) {
        match parse_dot(i) {
            Ok(_) => Ok(Query::Terms(terms)),
            Err(e) => Err(e),
        }
    } else {
        Err(Err::Error(("parse_query", ErrorKind::NoneOf)))
    }
}

named!(
    parse_fact<&str, Expr>,
    do_parse!(
        combined: parse_combined >>
        parse_dot >>
        (Expr::Fact (combined))
    )
);

named!(
    parse_rule<&str, Expr>,
    do_parse!(
        lhs: parse_combined >>
        tag!(":-") >>
        rhs: parse_combined_terms >>
        parse_dot >>
        (Expr::Rule { lhs, rhs })
    )
);

fn parse_expr(s: &str) -> IResult<&str, Expr> {
    parse_fact(s).or(parse_rule(s))
}

named!(pub parse_file_content<&str, (Vec<Expr>, &str)>, many_till!(parse_expr, eof!()));
