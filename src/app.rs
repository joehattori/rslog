use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::expr::{Query, Rule, Term, Variable};
use crate::parser::{parse_file_content, parse_query};
use crate::unifier::{unify, Constraint, Subst};

pub struct App {
    pub rules: Vec<Rule>,
    pub queue: VecDeque<QueueItem>,
    pub asked_vars: Vec<Variable>,
}

pub struct QueueItem {
    goals: Vec<Term>,
    subst: Subst,
}

pub struct Status {
    pub done: bool,
    pub subst: Subst,
}

impl App {
    pub fn new() -> App {
        App {
            rules: Vec::new(),
            queue: VecDeque::new(),
            asked_vars: Vec::new(),
        }
    }

    pub fn handle_query(&mut self, input: &str) -> Status {
        match parse_query(&input).expect("parse_query failed") {
            Query::File(file) => {
                let mut contents = fs::read_to_string(file).expect("No such file");
                contents.retain(|c| !c.is_whitespace());

                let (_, (rs, _)) = parse_file_content(&contents).expect("Error parsing file");
                let new_rules: Vec<Rule> = rs.iter().map(|r| r.add_prefix_to_term_var()).collect();
                println!("new_rules: {:?}", new_rules);
                self.rules.extend_from_slice(&new_rules);
                Status {
                    done: true,
                    subst: HashMap::new(),
                }
            }
            Query::Terms(terms) => {
                let all_free_vars = Term::free_vars_sum(&terms);
                self.asked_vars.append(&mut Term::free_vars_sum(&terms));
                self.queue.push_back(QueueItem {
                    goals: terms,
                    subst: HashMap::new(),
                });

                while let Some(QueueItem { mut goals, subst }) = self.queue.pop_front() {
                    println!(
                        "{}\n\tgoals: {:?}\n\trules: {:?}\n\tsubsts: {:?}",
                        self.queue.len(),
                        goals,
                        self.rules,
                        subst
                    );
                    match &goals.pop() {
                        None => {
                            return Status { done: false, subst };
                        }
                        Some(goal) => {
                            for rule in self.rules.iter() {
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
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                }

                Status {
                    done: true,
                    subst: HashMap::new(),
                }
            }
        }
    }
}
