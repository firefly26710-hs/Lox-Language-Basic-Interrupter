use crate::lex::token::AtomKind;

pub enum Expression{
    Atom(AtomKind),
    Op(char, Vec<Expression>)
}




    









