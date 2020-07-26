pub mod app;
pub mod expr;
pub mod parser;
pub mod unifier;
pub mod util;

use std::io::{stdin, stdout, Write};

use crate::app::{App, Status};
use crate::expr::Term;
use crate::unifier::search;

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

        let Status { done, subst } = app.handle_query(&input);
        if done {
            app.asked_vars.iter().for_each(|var| {
                search(&Term::Var(var.clone()), &subst)
                    .map(|t| println!("{} = {term}", var, term = t.to_string()));
            });
            app.asked_vars.clear();
        } else {
            //println!("subst {:?}", subst);
            //app.asked_vars.iter().for_each(|var| {
            //    println!("{} = {:?}", var, search(&Term::Var(var.clone()), &subst))
            //});
        }
    }
}
