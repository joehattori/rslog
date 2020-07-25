use nom::character::complete::{alpha1, alphanumeric1, digit1};
use nom::error::ErrorKind;
use nom::{delimited, do_parse, eof, is_not, many_till, map, map_res, named, opt, tag};
use nom::{Err, IResult};

use crate::expr::{Constant, Query, Rule, Term};
use crate::util::first_char;

named!(
    parse_const_int<&str, Constant>,
    do_parse!(
        i: map_res!(digit1, |s: &str| s.parse::<i32>()) >>
        (Constant::Int(i))
    )
);

named!(
    parse_const_str<&str, Constant>,
    do_parse!(
        s: delimited!(tag!("\""), is_not!("\""), tag!("\"")) >>
        (Constant::Str(s.to_string()))
    )
);

fn parse_const_name(s: &str) -> IResult<&str, Constant> {
    if first_char(s).is_lowercase() {
        alphanumeric1(s).and_then(|(i, n)| Ok((i, Constant::Name(n.to_string()))))
    } else {
        Err(Err::Error(("var", ErrorKind::AlphaNumeric)))
    }
}

fn parse_const(s: &str) -> IResult<&str, Term> {
    parse_const_int(s)
        .or(parse_const_str(s))
        .or(parse_const_name(s))
        .and_then(|(i, c)| Ok((i, Term::Const(c))))
}

fn parse_var(s: &str) -> IResult<&str, Term> {
    if first_char(s).is_uppercase() {
        alpha1(s).and_then(|(i, s)| Ok((i, Term::Var(s.to_string()))))
    } else {
        Err(Err::Error(("var", ErrorKind::Alpha)))
    }
}

fn parse_term(s: &str) -> IResult<&str, Term> {
    parse_combined(s).or(parse_const(s)).or(parse_var(s))
}

// TODO: bad code. Clean up with do_parse! and opt!
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

fn parse_functor(s: &str) -> IResult<&str, &str> {
    if first_char(s).is_lowercase() {
        alpha1(s)
    } else {
        Err(Err::Error(("functor", ErrorKind::Alpha)))
    }
}

named!(parse_paren_args<&str, Vec<Term>>, delimited!(tag!("("), parse_terms, tag!(")")));

named!(
    parse_combined<&str, Term>,
    do_parse!(
        functor: parse_functor >>
        args: parse_paren_args >>
        (Term::Combined{functor: functor.to_string(), args: args})
    )
);

named!(parse_file_name<&str, &str>, delimited!(tag!("['"), is_not!("']"), tag!("']")));

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
    parse_rhs<&str, Vec<Term>>,
    map!(
        opt!(
            do_parse!(
                tag!(":-") >>
                terms: parse_terms >>
                (terms)
            )
        ),
        |t| t.unwrap_or(Vec::new())
    )
);

named!(
    parse_rule<&str, Rule>,
    do_parse!(
        lhs: parse_term >>
        rhs: parse_rhs >>
        parse_dot >>
        (Rule { lhs, rhs })
    )
);

named!(pub parse_file_content<&str, (Vec<Rule>, &str)>, many_till!(parse_rule, eof!()));
