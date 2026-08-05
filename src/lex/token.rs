#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    
    
    // Atom
    Number(f64),
    Identifier(String),

    
    
    
    // Op
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
    SEMICOLON,
    COMMA,
    DOT,

    
    //KeyWord
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
    SEMICOLON,
    COMMA,
    DOT,
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
