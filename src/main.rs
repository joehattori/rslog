use crate::parser::parse_query;
use std::io::stdin;

const PROMPT: &'static str = "?- ";

fn main() {
    loop {
        print!("{}", PROMPT);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Invalid input");
        parse_query(&input);
    }
}
