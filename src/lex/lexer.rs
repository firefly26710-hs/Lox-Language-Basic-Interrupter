use crate::lex::token::{Token};
use crate::lex::token::AtomKind::{Number, Variable};
use crate::lex::token::Token::{Atom, Op, EOF};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer{
    input  : Vec<char>,
    pub tokens : Vec<Token>,
    current: usize,
    start  : usize
}

impl Lexer {
    pub fn new(input: &str)->Self{
        Lexer {
            input: input.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start  : 0
        }
    }

    pub fn scan_tokens(&mut self){
        while !self.finished(){
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(EOF);
    }

    fn scan_token(&mut self) {
            let c = self.advance();
            match c {
                '+' | '-' | '*' | '/' | '(' | ')' | '=' => self.tokens.push(Op(c)),
                'a'..='z' | 'A'..='Z' => self.variable(),
                '0'..='9' => self.number(),
                ' ' | '\r' |'\t'|'\n' => {}
                _ => panic!("No match this char"),
            }
    }


    fn number(&mut self) {
        while is_digit(self.peek()){self.current += 1}
        if self.peek() == '.' && is_digit(self.peek_next()){
            self.current += 1;
            while is_digit(self.peek()){ self.current += 1}
        }
        let string:String = self.input[self.start..self.current].iter().collect();
        let number = string.parse::<f64>();
        match number{
            Ok(val) => self.tokens.push(Token::Atom(Number(val))),
            Err(_) => panic!("Invalid Number")
        }
    }

    fn advance(&mut self) -> char{
        let c =self.input[self.current];
        self.current += 1;
        c
    }


    fn variable(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_'{ self.current += 1 }
        let variable:String = self.input[self.start..self.current].iter().collect();
        self.tokens.push(Atom(Variable(variable)))
    }


    fn peek(&self) -> char {
        if self.finished() { return '\0'; }
        self.input[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() {  return '\0' }
        self.input[self.current + 1]
    }

    fn finished(&self) -> bool {
        self.current >= self.input.len()
    }

    pub fn print(&mut self) {
        for token in &self.tokens {
            println!("{:?}", token) }
    }
}




fn is_digit( c : char) -> bool{
    c >= '0' && c <= '9'
}



