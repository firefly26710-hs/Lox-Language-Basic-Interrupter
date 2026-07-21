#[derive(Debug, Clone, PartialEq)]

pub enum Token{
    Atom(AtomKind),
    Op(String),
    EOF
}
#[derive(Debug, Clone, PartialEq)]

pub enum AtomKind{
    Number(f64),
    Variable(String)
}