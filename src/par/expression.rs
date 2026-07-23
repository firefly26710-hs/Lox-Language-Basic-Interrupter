use crate::lex::token::{AtomKind, Token};

#[derive(Debug)]
pub enum Expression {
    Binary  { left : usize, op : Token, right : usize},
    Group   { expr : usize},
    Variable{ variable : AtomKind},
    Unary { op: Token, right: usize },
}

