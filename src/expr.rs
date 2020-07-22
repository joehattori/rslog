pub enum Query {
    File(String),
    Terms(Vec<Term>),
}

impl Query {
    pub fn print(&self) {
        print!("Query(");
        match self {
            Query::File(s) => print!("File({})", s),
            Query::Terms(ts) => {
                for t in ts.iter() {
                    t.print();
                }
            }
        }
        print!(")");
    }
}

pub enum Term {
    Const(Constant),
    Var(String),
    Combined(CombinedTerm),
}

impl Term {
    pub fn print(&self) {
        print!("Term(");
        match self {
            Term::Const(c) => c.print(),
            Term::Var(s) => print!("Var({})", s),
            Term::Combined(ct) => ct.print(),
        }
        print!(")");
    }
}

pub enum Constant {
    Int(i32),
    Str(String),
    Name(String),
}

impl Constant {
    pub fn print(&self) {
        match self {
            Constant::Int(i) => print!("ConstInt({})", i),
            Constant::Str(s) => print!("ConstStr({})", s),
            Constant::Name(s) => print!("ConstName({})", s),
        }
    }
}

pub struct CombinedTerm {
    pub functor: String,
    pub args: Vec<Term>,
}

impl CombinedTerm {
    pub fn print(&self) {
        print!("CombinedTerm(functor: {}, args: [", self.functor);
        self.args.iter().for_each(|t| t.print());
        print!("])");
    }
}

pub struct Rule {
    pub lhs: CombinedTerm,
    pub rhs: Vec<CombinedTerm>,
}

pub enum Expr {
    Fact(CombinedTerm),
    Rule(Rule),
}
