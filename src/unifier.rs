use std::collections::HashMap;

use crate::expr::{Term, Variable};

pub type Subst = HashMap<Variable, Term>;

pub type Constraint = (Term, Term);

pub fn compose(s1: &Subst, s2: &Subst) -> Subst {
    let mut ret: Subst = s1.iter().map(|(v, t)| (v.clone(), t.subst(s2))).collect();
    ret.extend(s2.clone().into_iter());
    ret
}

pub fn unify(constraints: &mut Vec<Constraint>) -> Result<Subst, String> {
    match constraints.pop() {
        None => Ok(HashMap::new()),
        Some((left, right)) => {
            if left == right {
                return unify(constraints);
            }
            match (&left, &right) {
                (Term::Var(_), _) | (_, Term::Var(_)) => {
                    let (v, t) = if let Term::Var(v) = left {
                        (v, right)
                    } else if let Term::Var(v) = right {
                        (v, left)
                    } else {
                        panic!("This case won't happend.");
                    };
                    let sub: Subst = [(v.clone(), t.clone())].iter().cloned().collect();
                    let mut new_constraints: Vec<Constraint> = constraints
                        .iter()
                        .cloned()
                        .map(|(l, r)| (l.subst(&sub), r.subst(&sub)))
                        .collect();
                    unify(&mut new_constraints).map(|mut sub| {
                        sub.insert(v.clone(), t.clone());
                        sub
                    })
                }
                (Term::Const(_), _) | (_, Term::Const(_)) => Err("Unification error".to_string()),
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
                        Err("Unification error".to_string())
                    } else {
                        let mut new_constraints: Vec<Constraint> =
                            args1.iter().cloned().zip(args2.iter().cloned()).collect();
                        new_constraints.extend_from_slice(constraints);
                        unify(&mut new_constraints)
                    }
                }
            }
        }
    }
}

pub fn search(target: &Term, subst: &Subst) -> Term {
    match target {
        Term::Const(_) => target.clone(),
        Term::Var(v) => {
            let val = subst.get(v).expect(&format!("unbound variable {}", v));
            search(&val, subst)
        }
        Term::Combined { functor, args } => Term::Combined {
            functor: functor.clone(),
            args: args.iter().map(|arg| search(arg, subst)).collect(),
        },
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

        let mut constraints = vec![(
            Term::Combined {
                functor: "add".to_string(),
                args: vec![const_z.clone(), s_z.clone(), Term::Var("C".to_string())],
            },
            Term::Combined {
                functor: "add".to_string(),
                args: vec![const_z.clone(), s_z.clone(), s_z.clone()],
            },
        )];
        let expected: Subst = [("C".to_string(), s_z.clone())].iter().cloned().collect();
        assert_eq!(unify(&mut constraints), Ok(expected));
    }
}
