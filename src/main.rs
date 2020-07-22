pub mod expr;
pub mod parser;
pub mod util;

use crate::parser::parse_query;
use std::io::{stdin, stdout, Write};

const PROMPT: &'static str = "?- ";

fn main() {
    loop {
        print!("{}", PROMPT);
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Invalid input");
        match parse_query(&input.trim_end()) {
            Ok((_, q)) => q.print(),
            Err(e) => panic!("parse_query panic{}", e),
        }
        println!();
    }
}
