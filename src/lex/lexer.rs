use crate::lex::token::{Token};


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
        self.tokens.push(Token::EOF);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '+' => self.tokens.push(Token::PLUS),
            '-' => self.tokens.push(Token::MINUS),
            '*' => self.tokens.push(Token::MULTI),
            '/' => self.tokens.push(Token::DIVIDE),
            '=' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Token::EQUELEQUEL);
                    self.current += 1
                }
                self.tokens.push(Token::EQUEL)
            },
            '>' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Token::GREATEREUQEL);
                    self.current += 1
                }
                self.tokens.push(Token::GREATER)
            },
            '<' => {
                if matches!(self.peek(),'=') {
                    self.tokens.push(Token::LESSEREQUEL);
                    self.current += 1
                }
                self.tokens.push(Token::LESSER)
            },
            '(' => self.tokens.push(Token::LEFTPAREN),
            ')' => self.tokens.push(Token::RIGHTPAREN),
            '{' => self.tokens.push(Token::LEFTBRACE),
            '}' => self.tokens.push(Token::RIGHTBRACE),
            ';' => self.tokens.push(Token::SEMICOLON),
            ',' => self.tokens.push(Token::COMMA),
            '.' => self.tokens.push(Token::DOT),
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
            Ok(val) => self.tokens.push(Token::Number(val)),
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
            "let" => self.tokens.push(Token::LET),
            "fun" => self.tokens.push(Token::FUN),
            "for" => self.tokens.push(Token::FOR),
            "while" => self.tokens.push(Token::WHILE),
            "if" => self.tokens.push(Token::IF),
            "else" => self.tokens.push(Token::ELSE),
            "and" => self.tokens.push(Token::AND),
            "or" => self.tokens.push(Token::OR),
            "true" => self.tokens.push(Token::TRUE),
            "false" => self.tokens.push(Token::FALSE),
            "null" => self.tokens.push(Token::NULL),
            "struct" => self.tokens.push(Token::STRUCT),
            "print" => self.tokens.push(Token::PRINT),
            "return" => self.tokens.push(Token::RETURN),
            _ => self.tokens.push(Token::Identifier(slice))
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






