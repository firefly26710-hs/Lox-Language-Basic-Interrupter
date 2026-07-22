use crate::lex::token::AtomKind;

#[derive(Debug)]
pub enum Expression{
    Atom(AtomKind),
    Op(char, Vec<Expression>)
}

impl Expression {
    pub fn eval(&self) -> f64 {
        match self {
            Expression::Atom(AtomKind::Number(n)) => *n,
            Expression::Op('+', exprs) => exprs[0].eval() + exprs[1].eval(),
            Expression::Op('-', exprs) => exprs[0].eval() - exprs[1].eval(),
            Expression::Op('*', exprs) => exprs[0].eval() * exprs[1].eval(),
            Expression::Op('/', exprs) => exprs[0].eval() / exprs[1].eval(),
            _ => panic!("Unsupported expression"),
        }
    }
}







