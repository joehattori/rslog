#[derive(Debug)]
pub enum Query {
    File(String),
    Terms(Vec<Term>),
}

#[derive(Clone, Debug)]
pub enum Term {
    Const(Constant),
    Var(String),
    Combined(CombinedTerm),
}

#[derive(Clone, Debug)]
pub enum Constant {
    Int(i32),
    Str(String),
    Name(String),
}

#[derive(Clone, Debug)]
pub struct CombinedTerm {
    pub functor: String,
    pub args: Vec<Term>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Fact(CombinedTerm),
    Rule {
        lhs: CombinedTerm,
        rhs: Vec<CombinedTerm>,
    },
}
