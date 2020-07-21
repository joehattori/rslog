use nom::{
    bytes::complete::{is_not, tag},
    sequence::delimited,
    IResult,
};

use crate::expr::{Constant, Query};

pub fn parse_query(input: &str) -> Query {
    if input.starts_with("[") {
        let (i, o) = dig_file_name(input).expect("Error while parsing query");
        if i == "." {
            Query::File(o.to_string())
        } else {
            panic!("Should end with '.'");
        }
    } else {
        parse_terms(input)
    }
}

fn dig_file_name(input: &str) -> IResult<&str, &str> {
    delimited(tag("['"), is_not("']"), tag("']"))(input)
}

fn parse_terms(s: &str) -> Query {
    let v = Vec::new();
    Query::Terms(v)
}

fn parse_const(s: &str) -> Option<Constant> {
    parse_const_int(s)
        .or(parse_const_str(s))
        .or(Some(Constant::Name(s.to_string())))
}

fn parse_const_int(s: &str) -> Option<Constant> {
    s.parse::<i64>().ok().map(|i| Constant::Int(i))
}

fn parse_const_str(s: &str) -> Option<Constant> {
    let double_quotes = "\"";
    if s.starts_with(double_quotes) && s.ends_with(double_quotes) {
        Some(Constant::Str(s[1..s.len() - 2].to_string()))
    } else {
        None
    }
}
