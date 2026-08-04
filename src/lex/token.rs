#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Atom(AtomKind),
    Op(OpKind),
    KeyWord(KeyWordKind),
    EOF
}
#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind{
    Number(f64),
    Idenitifer(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpKind{
    PLUS,
    MINUS,
    MULTI,
    DIVIDE,
    EQUEL,
    EQUELEQUEL,
    NOT,
    NOTEQUEL,
    GREATER,
    GREATEREUQEL,
    LESSER,
    LESSEREQUEL,
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    SEMICOLON,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyWordKind{
    LET,
    FUN,
    FOR,
    WHILE,
    IF,
    ELSE,
    OR,
    AND,
    TRUE,
    FALSE,
    NULL,
    STRUCT,
    PRINT,
    RETURN,
}