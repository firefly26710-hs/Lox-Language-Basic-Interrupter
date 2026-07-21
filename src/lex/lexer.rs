use crate::lex::token::{AtomKind, Token};
use crate::lex::token::Token::Op;

#[derive(Debug, Clone, PartialEq)]

pub struct Lexer{
    input  : Vec<char>,
    tokens : Vec<Token>,
    current: usize
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            tokens: Vec::new(),
            current: 0
        }
    }

    pub fn token_pattern(&mut self) {
        while !self.current > self.input.len() {
            match self.input[self.current] {
                //'+' | '-' | '*' | '/' | '(' | ')' => self.tokens.push(Op(c.to_string())),
                // 'a'..='z' | 'A'..='Z' => self.variable()
                // '1'..='9' => self.number();
                _ => todo!(),
            }
        }
    }


    fn number(&mut self) {
        let peek_next = self.peek_next();
        if self.is_alpha(peek_next){
            panic!("Variable Problem in {}", peek_next)
        }
    }


    fn variable(&self){
        
    }

    fn peek(&mut self)->char{
         self.input[self.current]
    }

    fn peek_next(&mut self)->char{
        if !self.current > self.input.len(){
           return self.input[self.current + 1]
        }
        panic!("Out Of Index")
    }

    pub fn is_op(&self, c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' => true,
            _ => false
        }
    }

    pub fn is_number(&mut self, c : char) -> bool{
        match c{
            '1'..='9' => true,
            _     => false
        }
    }

    pub fn is_alpha(&mut self, c : char) -> bool{
        match c{
            'a'..='z'|'A'..='Z' => true,
            _                   => false
        }
    }


    pub fn print(&mut self) {
        for token in &self.tokens {
            println!("{:?}", token)
        }
    }
}


