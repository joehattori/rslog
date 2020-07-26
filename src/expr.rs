use std::collections::HashMap;

use crate::unifier::{compose, Subst};

#[derive(Debug)]
pub enum Query {
    File(String),
    Terms(Vec<Term>),
}

pub type Variable = String;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Term {
    Const(Constant),
    Var(Variable),
    Combined { functor: String, args: Vec<Term> },
}

impl Term {
    pub fn free_vars(&self) -> Vec<Variable> {
        match self {
            Term::Const(_) => Vec::new(),
            Term::Var(v) => vec![v.clone()],
            Term::Combined { functor: _, args } => args
                .iter()
                .fold(Vec::new(), |ret, term| [ret, term.free_vars()].concat()),
        }
    }

    pub fn free_vars_sum(terms: &Vec<Term>) -> Vec<Variable> {
        terms
            .iter()
            .map(|term| term.free_vars())
            .flatten()
            .collect()
    }

    pub fn has_free_var(&self) -> bool {
        match self {
            Term::Const(_) => false,
            Term::Var(_) => true,
            Term::Combined { functor: _, args } => args.iter().any(|arg| arg.has_free_var()),
        }
    }

    // map of vars in self to form term.
    pub fn var_to_term_map(&self, term: &Term) -> Option<Subst> {
        match self {
            Term::Var(v) => {
                let mut map = HashMap::new();
                map.insert(v.clone(), term.clone());
                Some(map)
            }
            Term::Const(_) => Some(HashMap::new()),
            Term::Combined { functor, args } => match term {
                Term::Combined {
                    functor: functor2,
                    args: args2,
                } => {
                    if functor != functor2 {
                        None
                    } else {
                        args.iter()
                            .zip(args2.iter())
                            .fold(Some(HashMap::new()), |ret, (t1, t2)| {
                                let mut sub_acum = ret?;
                                match t1.var_to_term_map(t2) {
                                    None => Some(sub_acum),
                                    Some(sub) => {
                                        let mut valid = true;
                                        sub.iter().for_each(|(v, t)| {
                                            match sub_acum.clone().get(v) {
                                                None => {
                                                    sub_acum.insert(v.clone(), t.clone());
                                                }
                                                Some(val) => match (val, t) {
                                                    (Term::Var(_), Term::Var(_)) => {
                                                        if val != t {
                                                            valid = false;
                                                        }
                                                    }
                                                    (Term::Var(_), _) => {
                                                        sub_acum.insert(v.clone(), t.clone());
                                                    }
                                                    (_, Term::Var(_)) => {
                                                        sub_acum.insert(v.clone(), val.clone());
                                                    }
                                                    _ => {
                                                        if val != t {
                                                            valid = false;
                                                        }
                                                    }
                                                },
                                            }
                                        });
                                        if valid {
                                            Some(sub_acum)
                                        } else {
                                            None
                                        }
                                    }
                                }
                            })
                    }
                }
                _ => Some(HashMap::new()),
            },
        }
    }

    pub fn subst(&self, map: &Subst) -> Term {
        match self {
            Term::Const(_) => self.clone(),
            Term::Var(v) => match map.get(v) {
                Some(term) => term.clone(),
                None => self.clone(),
            },
            Term::Combined { functor, args } => Term::Combined {
                functor: functor.clone(),
                args: args.iter().map(|term| term.subst(&map)).collect(),
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Term::Const(c) => match c {
                Constant::Int(i) => format!("{}", i),
                Constant::Str(s) => format!("\"{}\"", s),
                Constant::Name(name) => name.clone(),
            },
            Term::Var(v) => v.clone(),
            Term::Combined { functor, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", functor, args_str)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Constant {
    Int(i32),
    Str(String),
    Name(String),
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub lhs: Term,
    pub rhs: Vec<Term>,
}

impl Rule {
    pub fn instantiate(&self, vars_count: &mut i32, map: &Subst) -> Rule {
        let mut free_vars = vec![self.lhs.free_vars(), Term::free_vars_sum(&self.rhs)].concat();
        free_vars.sort();
        free_vars.dedup();
        let number_map: Subst = free_vars
            .iter()
            .map(|v| {
                *vars_count = *vars_count + 1;
                (v.clone(), Term::Var(format!("{}", vars_count)))
            })
            .collect();
        let sub = compose(&number_map, map);
        Rule {
            lhs: self.lhs.subst(&sub),
            rhs: self.rhs.iter().map(|term| term.subst(&sub)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_to_term_map() {
        // rule: add(s(X), Y, s(Z)) :- add(X, Y, Z). query: add(s(z), s(z), X).
        let const_z = Term::Const(Constant::Name("z".to_string()));
        let s_z = Term::Combined {
            functor: "s".to_string(),
            args: vec![const_z.clone()],
        };
        let rule_lhs = Term::Combined {
            functor: "add".to_string(),
            args: vec![
                Term::Combined {
                    functor: "s".to_string(),
                    args: vec![Term::Var("X".to_string())],
                },
                Term::Var("Y".to_string()),
                Term::Combined {
                    functor: "s".to_string(),
                    args: vec![Term::Var("Z".to_string())],
                },
            ],
        };
        let query = Term::Combined {
            functor: "add".to_string(),
            args: vec![s_z.clone(), s_z.clone(), Term::Var("X".to_string())],
        };
        let expected = [
            ("X".to_string(), const_z.clone()),
            (
                "Y".to_string(),
                Term::Combined {
                    functor: "s".to_string(),
                    args: vec![const_z.clone()],
                },
            ),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(rule_lhs.var_to_term_map(&query), Some(expected));

        // rule: add(z, Y, Y). query: add(z, X, s(z)).
        let fact = Term::Combined {
            functor: "add".to_string(),
            args: vec![
                const_z.clone(),
                Term::Var("Y".to_string()),
                Term::Var("Y".to_string()),
            ],
        };
        let query = Term::Combined {
            functor: "add".to_string(),
            args: vec![const_z.clone(), Term::Var("X".to_string()), s_z.clone()],
        };
        let expected = [(
            "Y".to_string(),
            Term::Combined {
                functor: "s".to_string(),
                args: vec![const_z.clone()],
            },
        )]
        .iter()
        .cloned()
        .collect();
        assert_eq!(fact.var_to_term_map(&query), Some(expected));

        // TODO: test for father(X, Y) :- parent(X, Y), male(Y).
    }
}
