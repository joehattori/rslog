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

    pub fn subst(&self, subst: &Subst) -> Term {
        match self {
            Term::Const(_) => self.clone(),
            Term::Var(v) => {
                if *v == subst.var {
                    subst.term.clone()
                } else {
                    self.clone()
                }
            }
            Term::Combined { functor, args } => Term::Combined {
                functor: functor.clone(),
                args: args.iter().map(|t| t.subst(subst)).collect(),
            },
        }
    }

    pub fn subst_all(&self, substs: &Vec<Subst>) -> Term {
        substs
            .iter()
            .fold(self.clone(), |term, sub| term.subst(&sub))
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
