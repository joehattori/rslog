pub mod expr;
pub mod parser;
pub mod util;

use crate::expr::Query;
use crate::parser::{parse_file_content, parse_query};
use std::fs;
use std::io::{stdin, stdout, Write};

const PROMPT: &'static str = "?- ";

fn main() {
    loop {
        print!("\n{}", PROMPT);
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Invalid input");
        input.retain(|c| !c.is_whitespace());
        if input == "exit" {
            break;
        }
        match parse_query(&input).expect("parse_query failed") {
            Query::File(f) => {
                let mut contents = fs::read_to_string(f).expect("No such file");
                contents.retain(|c| !c.is_whitespace());
                let (_, (exprs, _)) = parse_file_content(&contents).expect("Error parsing file");
                exprs.iter().for_each(|expr| println!("{:?}", expr));
            }
            Query::Terms(t) => println!("{:?}", t),
        }
    }
}
