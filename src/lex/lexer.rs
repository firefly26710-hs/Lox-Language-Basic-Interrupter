use crate::lex::token::Token;
pub struct Lexer{
    tokens : Vec<Token>
}

impl Lexer{
    fn new() -> Self{
        Lexer{tokens : Vec::new()}
    }
}