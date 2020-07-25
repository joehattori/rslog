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

    pub fn reload(&self) -> Term {
        let subs: Vec<Subst> = self
            .free_vars()
            .iter()
            .map(|v| Subst {
                var: v.clone(),
                term: Term::Var(v.clone()),
            })
            .collect();
        self.subst_all(&subs)
    }

    fn add_prefix_to_var(&self) -> Term {
        match self {
            Term::Var(s) => Term::Var("rule_".to_string() + s),
            _ => self.clone(),
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
    pub fn reload(&self) -> Rule {
        Rule {
            lhs: self.lhs.reload(),
            rhs: self.rhs.iter().map(|t| t.reload()).collect(),
        }
    }

    // add prefix "rule_" to vars in term to avoid name collision between query and rule.
    pub fn add_prefix_to_term_var(&self) -> Rule {
        Rule {
            lhs: self.lhs.add_prefix_to_var(),
            rhs: self.rhs.iter().map(|t| t.add_prefix_to_var()).collect(),
        }
    }
}
