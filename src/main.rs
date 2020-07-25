pub mod app;
pub mod expr;
pub mod parser;
pub mod unifier;
pub mod util;

use std::io::{stdin, stdout, Write};

use crate::app::{App, Status};

const PROMPT: &'static str = "?- ";
const HALT_MESSAGE: &'static str = "halt.";

fn main() {
    let mut app = App::new();
    loop {
        print!("\n{}", PROMPT);
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Invalid input");
        input.retain(|c| !c.is_whitespace());

        if input == HALT_MESSAGE {
            break;
        }

        let Status { done, var_to_term } = app.handle_query(&input);
        if done {
            println!("true.");
            println!("var_to_term {:?}", var_to_term);
            //app.asked_vars
            //.iter()
            //.for_each(|var| println!("{} = {:?}", var, substs.get(var)));
            app.asked_vars.clear();
        } else {
            println!("{:?}", var_to_term);
            //app.asked_vars
            //.iter()
            //.for_each(|var| println!("{} = {:?}", var, substs.get(var)));
        }
    }
}
