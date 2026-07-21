pub enum Token{
    Atom(AtomKind),
    Operator(String)
}

enum AtomKind{
    Number(f64),
    Bool(bool)
}