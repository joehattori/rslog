pub mod expr;
pub mod parser;
pub mod unifier;
pub mod util;

use crate::expr::{Query, Rule, Term, Variable};
use crate::parser::{parse_file_content, parse_query};
use crate::unifier::{unify, Constraint, Subst};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{stdin, stdout, Write};

const PROMPT: &'static str = "?- ";
const HALT_MESSAGE: &'static str = "halt.";

struct QueueItem {
    goals: Vec<Term>,
    substs: Vec<Subst>,
}

fn main() {
    let mut rules: Vec<Rule> = Vec::new();
    let mut queue: VecDeque<QueueItem> = VecDeque::new();
    loop {
        print!("\n{}", PROMPT);
        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Invalid input");
        input.retain(|c| !c.is_whitespace());

        if input == HALT_MESSAGE {
            break;
        }

        match parse_query(&input).expect("parse_query failed") {
            Query::File(file) => {
                let mut contents = fs::read_to_string(file).expect("No such file");
                contents.retain(|c| !c.is_whitespace());

                let (_, (rs, _)) = parse_file_content(&contents).expect("Error parsing file");
                rs.iter().for_each(|rule| println!("{:?}", rule));
                rules.extend_from_slice(&rs);
            }
            Query::Terms(terms) => {
                queue.push_back(QueueItem {
                    goals: terms,
                    substs: Vec::new(),
                });

                while let Some(QueueItem { mut goals, substs }) = queue.pop_front() {
                    match &goals.pop() {
                        None => {
                            let var_to_term: HashMap<Variable, Term> = Term::free_vars_sum(&goals)
                                .iter()
                                .cloned()
                                .map(|var| (var.clone(), Term::Var(var).subst_all(&substs)))
                                .collect();

                            if var_to_term.iter().any(|(_, term)| term.has_free_var()) {
                                continue;
                            }
                            println!("DONE");
                            println!("{:?}", var_to_term);
                        }
                        Some(goal) => {
                            for rule in rules.iter().cloned() {
                                match (goal, &rule.lhs) {
                                    (
                                        Term::Combined {
                                            functor: f1,
                                            args: args1,
                                        },
                                        Term::Combined {
                                            functor: f2,
                                            args: args2,
                                        },
                                    ) => {
                                        if f1 != f2 || args1.len() != args2.len() {
                                            continue;
                                        }

                                        let mut constraints: Vec<Constraint> = args1
                                            .iter()
                                            .zip(args2.iter())
                                            .map(|(arg1, arg2)| {
                                                (arg1.subst_all(&substs), arg2.clone())
                                            })
                                            .collect();

                                        if let Some(mut new_substs) = unify(&mut constraints) {
                                            let mut goals_to_append: Vec<Term> =
                                                rule.rhs.iter().cloned().collect::<Vec<Term>>();

                                            let mut goals = goals.clone();
                                            goals.append(&mut goals_to_append);
                                            let mut substs = substs.clone();
                                            substs.append(&mut new_substs);
                                            queue.push_back(QueueItem { goals, substs });
                                        }
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
