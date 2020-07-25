use crate::expr::{Term, Variable};

#[derive(Clone, Debug, PartialEq)]
pub struct Subst {
    pub var: Variable,
    pub term: Term,
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

                        let mut new_constraints = constraints
                            .iter()
                            .map(|con| (con.0.subst(&sub), con.1.subst(&sub)))
                            .collect();

                        unify(&mut new_constraints).and_then(|mut ret| {
                            ret.push(sub);
                            Some(ret)
                        })
                    }
                    (Term::Const(_), _) | (_, Term::Const(_)) => None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Constant;

    #[test]
    fn test_unify() {
        let const_z = Term::Const(Constant::Name("z".to_string()));
        let s_z = Term::Combined {
            functor: "s".to_string(),
            args: vec![const_z.clone()],
        };

        let mut constraints = vec![
            (
                s_z.clone(),
                Term::Combined {
                    functor: "s".to_string(),
                    args: vec![Term::Var("rule_X".to_string())],
                },
            ),
            (s_z.clone(), Term::Var("rule_Y".to_string())),
            (
                Term::Var("X".to_string()),
                Term::Combined {
                    functor: "s".to_string(),
                    args: vec![Term::Var("Z".to_string())],
                },
            ),
        ];
        assert_eq!(
            unify(&mut constraints),
            Some(vec![Subst {
                var: "X".to_string(),
                term: Term::Combined {
                    functor: "s".to_string(),
                    args: vec![s_z.clone()]
                },
            },]),
        );
    }
}
