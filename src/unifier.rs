use std::collections::HashMap;

use crate::expr::{Term, Variable};

pub type Subst = HashMap<Variable, Term>;

pub type Constraint = (Term, Term);

// TODO
pub fn compose(s1: &Subst, s2: &Subst) -> Subst {
    s1.clone()
}

// TODO
pub fn unify(constraints: &mut Vec<Constraint>) -> Option<Subst> {
    match constraints.pop() {
        None => Some(HashMap::new()),
        Some((left, right)) => Some(HashMap::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Constant;

    #[test]
    fn test_unify() {
        //let const_z = Term::Const(Constant::Name("z".to_string()));
        //let s_z = Term::Combined {
        //functor: "s".to_string(),
        //args: vec![const_z.clone()],
        //};

        //let mut constraints = vec![
        //(
        //s_z.clone(),
        //Term::Combined {
        //functor: "s".to_string(),
        //args: vec![Term::Var("rule_X".to_string())],
        //},
        //),
        //(s_z.clone(), Term::Var("rule_Y".to_string())),
        //(
        //Term::Var("X".to_string()),
        //Term::Combined {
        //functor: "s".to_string(),
        //args: vec![Term::Var("Z".to_string())],
        //},
        //),
        //];
        //assert_eq!(
        //unify(&mut constraints),
        //Some(vec![Subst {
        //var: "X".to_string(),
        //term: Term::Combined {
        //functor: "s".to_string(),
        //args: vec![s_z.clone()]
        //},
        //},]),
        //);
    }
}
