use crate::lex::token::{AtomKind, Token};

#[derive(Debug)]
pub enum Expression {
    Binary  { left : usize, op : Token, righr : usize},
    Group   { expr : usize},
    Variable{ variable : AtomKind},
    Unary
}
