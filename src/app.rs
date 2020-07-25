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
    substs: Vec<Subst>,
}

pub struct Status {
    pub done: bool,
    pub var_to_term: HashMap<Variable, Term>,
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
                    var_to_term: HashMap::new(),
                }
            }
            Query::Terms(terms) => {
                let all_free_vars = Term::free_vars_sum(&terms);
                self.asked_vars.append(&mut Term::free_vars_sum(&terms));
                self.queue.push_back(QueueItem {
                    goals: terms,
                    substs: Vec::new(),
                });

                while let Some(QueueItem { mut goals, substs }) = self.queue.pop_front() {
                    println!(
                        "{}\n\tgoals: {:?}\n\trules: {:?}\n\tsubsts: {:?}",
                        self.queue.len(),
                        goals,
                        self.rules,
                        substs
                    );
                    match &goals.pop() {
                        None => {
                            let var_to_term: HashMap<Variable, Term> = all_free_vars
                                .iter()
                                .cloned()
                                .map(|var| (var.clone(), Term::Var(var).subst_all(&substs)))
                                .collect();

                            if var_to_term.iter().any(|(_, term)| term.has_free_var()) {
                                continue;
                            }
                            println!("var_to_term {:?}", var_to_term);
                            println!("goals {:?}", goals);

                            return Status {
                                done: false,
                                var_to_term,
                            };
                        }
                        Some(goal) => {
                            for rule in self.rules.iter() {
                                println!("rule {:?}\ngoal {:?}", rule, goal);
                                //let rule = &rule.reload();
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

                                        println!("const: {:?}", constraints);
                                        if let Some(mut new_substs) = unify(&mut constraints) {
                                            println!("unifiable result: {:?}", new_substs);
                                            let mut goals_to_append: Vec<Term> =
                                                rule.rhs.iter().cloned().collect();

                                            let mut goals = goals.clone();
                                            goals.append(&mut goals_to_append);
                                            let mut substs = substs.clone();
                                            substs.append(&mut new_substs);
                                            self.queue.push_back(QueueItem { goals, substs });
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
                    var_to_term: HashMap::new(),
                }
            }
        }
    }
}
