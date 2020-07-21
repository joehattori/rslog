pub enum Query {
    File(String),
    Terms(Vec<Term>),
}

pub enum Term {
    Const(Constant),
    Var(String),
    Combined(CombinedTerms),
}

pub enum Constant {
    Int(i64),
    Str(String),
    Name(String),
}

pub struct CombinedTerms {
    pub functor: String,
    pub args: Vec<Term>,
}

pub struct Rule {
    pub lhs: CombinedTerms,
    pub rhs: Vec<CombinedTerms>,
}

pub enum Expr {
    Fact(CombinedTerms),
    Rule(Rule),
}
