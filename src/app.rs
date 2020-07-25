use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::expr::{Query, Rule, Term, Variable};
use crate::parser::{parse_file_content, parse_query};
use crate::unifier::{compose, unify, Subst};

pub struct App {
    pub rules: Vec<Rule>,
    pub queue: VecDeque<QueueItem>,
    pub asked_vars: Vec<Variable>,
    pub vars_count: i32,
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
            vars_count: 0,
        }
    }

    pub fn handle_query(&mut self, input: &str) -> Status {
        match parse_query(&input).expect("parse_query failed") {
            Query::File(file) => {
                let mut contents = fs::read_to_string(file).expect("No such file");
                contents.retain(|c| !c.is_whitespace());

                let (_, (new_rules, _)) =
                    parse_file_content(&contents).expect("Error parsing file");
                self.rules.extend_from_slice(&new_rules);
                Status {
                    done: true,
                    subst: HashMap::new(),
                }
            }
            Query::Terms(goals) => {
                self.asked_vars.append(&mut Term::free_vars_sum(&goals));
                self.queue.push_back(QueueItem {
                    goals,
                    subst: HashMap::new(),
                });

                while let Some(QueueItem { mut goals, subst }) = self.queue.pop_front() {
                    println!("\ngoals: {:?}\nsubst: {:?}", goals, subst);
                    match &goals.pop() {
                        None => {
                            return Status { done: false, subst };
                        }
                        Some(goal) => {
                            for rule in self.rules.iter() {
                                match rule.lhs.var_to_term_map(&goal) {
                                    None => continue,
                                    Some(map) => {
                                        let new_rule = rule.instantiate(&mut self.vars_count, &map);
                                        let mut constraints =
                                            vec![(goal.clone(), new_rule.lhs.clone())];
                                        match unify(&mut constraints) {
                                            Err(_) => continue,
                                            Ok(sub) => {
                                                let new_subst = compose(&sub, &subst);
                                                let goals_to_append = new_rule
                                                    .rhs
                                                    .iter()
                                                    .map(|t| t.subst(&new_subst))
                                                    .collect();
                                                let new_goals =
                                                    vec![goals.clone(), goals_to_append].concat();
                                                self.queue.push_back(QueueItem {
                                                    goals: new_goals,
                                                    subst: new_subst,
                                                });
                                            }
                                        }
                                    }
                                };
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
