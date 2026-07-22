#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Atom(AtomKind),
    Op(char),
    EOF
}
#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind{
    Number(f64),
    Variable(String)
}