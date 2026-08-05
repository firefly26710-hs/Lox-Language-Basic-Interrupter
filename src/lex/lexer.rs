use crate::lex::token::KeyWordKind;
use crate::lex::token::Token::KeyWord;
use crate::lex::token::{OpKind};
use crate::lex::token::{Token};
use crate::lex::token::AtomKind::{Number, Idenitifer};
use crate::lex::token::Token::{Atom, Op, EOF};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer{
    input  : Vec<char>,
    pub tokens : Vec<Token>,
    current: usize,
    start  : usize
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.finished() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(EOF);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '+' => self.tokens.push(Op(OpKind::PLUS)),
            '-' => self.tokens.push(Op(OpKind::MINUS)),
            '*' => self.tokens.push(Op(OpKind::MULTI)),
            '/' => self.tokens.push(Op(OpKind::DIVIDE)),
            '=' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Op(OpKind::EQUELEQUEL));
                    self.current += 1
                }
                self.tokens.push(Op(OpKind::EQUEL))
            },
            '>' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Op(OpKind::GREATEREUQEL));
                    self.current += 1
                }
                self.tokens.push(Op(OpKind::GREATER))
            },
            '<' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Op(OpKind::LESSEREQUEL));
                    self.current += 1
                }
                self.tokens.push(Op(OpKind::LESSER))
            },
            '(' => self.tokens.push(Op(OpKind::LEFTPAREN)),
            ')' => self.tokens.push(Op(OpKind::RIGHTPAREN)),
            '{' => self.tokens.push(Op(OpKind::LEFTBRACE)),
            '}' => self.tokens.push(Op(OpKind::RIGHTBRACE)),
            ';' => self.tokens.push(Op(OpKind::SEMICOLON)),
            ',' => self.tokens.push(Op(OpKind::COMMA)),
            '.' => self.tokens.push(Op(OpKind::DOT)),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            '0'..='9' => self.number(),
            ' ' | '\r' | '\t' | '\n' => {}
            _ => panic!("No match this char"),
        }
    }


    fn number(&mut self) {
        while is_digit(self.peek()) { self.current += 1 }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.current += 1;
            while is_digit(self.peek()) { self.current += 1 }
        }
        let slice: String = self.input[self.start..self.current].iter().collect();
        let number = slice.parse::<f64>();
        match number {
            Ok(val) => self.tokens.push(Atom(Number(val))),
            Err(_) => panic!("Invalid Number")
        }
    }

    fn advance(&mut self) -> char {
        let c = self.input[self.current];
        self.current += 1;
        c
    }


    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.current += 1 }

        let slice: String = self.input[self.start..self.current].iter().collect();
        match slice.as_str() {
            "let" => self.tokens.push(KeyWord(KeyWordKind::LET)),
            "fun" => self.tokens.push(KeyWord(KeyWordKind::FUN)),
            "for" => self.tokens.push(KeyWord(KeyWordKind::FOR)),
            "while" => self.tokens.push(KeyWord(KeyWordKind::WHILE)),
            "if" => self.tokens.push(KeyWord(KeyWordKind::IF)),
            "else" => self.tokens.push(KeyWord(KeyWordKind::ELSE)),
            "and" => self.tokens.push(KeyWord(KeyWordKind::AND)),
            "or" => self.tokens.push(KeyWord(KeyWordKind::OR)),
            "true" => self.tokens.push(KeyWord(KeyWordKind::TRUE)),
            "false" => self.tokens.push(KeyWord(KeyWordKind::FALSE)),
            "null" => self.tokens.push(KeyWord(KeyWordKind::NULL)),
            "struct" => self.tokens.push(KeyWord(KeyWordKind::LET)),
            "print" => self.tokens.push(KeyWord(KeyWordKind::PRINT)),
            "return" => self.tokens.push(KeyWord(KeyWordKind::RETURN)),
            _ => self.tokens.push(Atom(Idenitifer(slice)))
        }
    }


    fn peek(&self) -> char {
        if self.finished() { return '\0'; }
        self.input[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.input.len() { return '\0' }
        self.input[self.current + 1]
    }

    fn finished(&self) -> bool {
        self.current >= self.input.len()
    }

    pub fn print(&mut self) {
        println!("{:?}", self.tokens);
    }
    
}
fn is_digit( c : char) -> bool{
    c >= '0' && c <= '9'
}

#[test]
fn lexer_var(){
    let input = "let x = 5;";
    let mut lexer = Lexer::new(&input);
    println!("{:?}", input);
    lexer.scan_tokens();
    lexer.print()
}

#[test]
fn lexer_fun(){
    let input = "fun add(a, b)";
    let mut lexer = Lexer::new(&input);
    println!("{:?}", input);
    lexer.scan_tokens();
    lexer.print()
}

#[test]
fn lexer_con(){
    let input = "if a > 0 {} else {}";
    let mut lexer = Lexer::new(&input);
    println!("{:?}", input);
    lexer.scan_tokens();
    lexer.print()
}






