use crate::expr::{Term, Variable};

#[derive(Clone)]
pub struct Subst {
    pub var: Variable,
    pub term: Term,
}

impl Subst {
    pub fn apply(&self) {}
}

pub type Constraint = (Term, Term);

pub fn unify(constraints: &mut Vec<Constraint>) -> Option<Vec<Subst>> {
    match constraints.pop() {
        None => Some(Vec::new()),
        Some((left, right)) => {
            if left == right {
                unify(constraints)
            } else {
                match (&left, &right) {
                    (Term::Const(_), _) | (_, Term::Const(_)) => None,
                    (Term::Var(_), _) | (_, Term::Var(_)) => {
                        let sub = if let Term::Var(v) = left {
                            Subst {
                                var: v.clone(),
                                term: right,
                            }
                        } else if let Term::Var(v) = right {
                            Subst {
                                var: v.clone(),
                                term: left,
                            }
                        } else {
                            panic!("This case won't hold.")
                        };
                        match unify(
                            &mut constraints
                                .iter()
                                .map(|con| (con.0.subst(&sub), con.1.subst(&sub)))
                                .collect(),
                        ) {
                            Some(mut ret) => {
                                ret.push(sub);
                                Some(ret)
                            }
                            None => None,
                        }
                    }
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
                        if f1 != f2 {
                            None
                        } else if args1.len() != args2.len() {
                            None
                        } else {
                            let mut consts_to_append: Vec<Constraint> = args1
                                .iter()
                                .zip(args2.iter())
                                .map(|(a1, a2)| (a1.clone(), a2.clone()))
                                .collect();
                            constraints.append(&mut consts_to_append);
                            unify(constraints)
                        }
                    }
                }
            }
        }
    }
}
