#[derive(Debug)]
pub enum Query {
    File(String),
    Terms(Vec<Term>),
}

#[derive(Clone, Debug)]
pub enum Term {
    Const(Constant),
    Var(String),
    Combined { functor: String, args: Vec<Term> },
}

#[derive(Clone, Debug)]
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
