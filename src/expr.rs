use std::collections::HashMap;

use crate::unifier::Subst;

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

    fn add_prefix_to_var(&self) -> Term {
        match self {
            Term::Var(s) => Term::Var("rule_".to_string() + s),
            _ => self.clone(),
        }
    }

    // map of vars in self to actuate term.
    pub fn var_to_term_map(&self, term: &Term) -> Option<Subst> {
        match self {
            Term::Var(v) => {
                let mut map = HashMap::new();
                map.insert(v.clone(), term.clone());
                Some(map)
            }
            Term::Const(_) => None,
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
                                    Some(sub) => {
                                        let has_different_value = sub.iter().any(|(v, t)| {
                                            sub_acum.contains_key(v) && sub_acum.get(v) != Some(t)
                                        });
                                        if has_different_value {
                                            None
                                        } else {
                                            sub.iter().for_each(|(v, t)| {
                                                sub_acum.insert(v.clone(), t.clone());
                                            });
                                            Some(sub_acum)
                                        }
                                    }
                                    None => Some(sub_acum),
                                }
                            })
                    }
                }
                _ => Some(HashMap::new()),
            },
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
    // add prefix "rule_" to vars in term to avoid name collision between query and rule.
    pub fn add_prefix_to_term_var(&self) -> Rule {
        Rule {
            lhs: self.lhs.add_prefix_to_var(),
            rhs: self.rhs.iter().map(|t| t.add_prefix_to_var()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_to_term_map() {
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
    }
}
