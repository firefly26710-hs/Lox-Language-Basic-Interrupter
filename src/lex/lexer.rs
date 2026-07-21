use crate::lex::token::{AtomKind, Token};
use crate::lex::token::Token::{Atom, Op};
#[derive(Debug, Clone, PartialEq)]

pub struct Lexer{
    tokens : Vec<Token>
}

impl Lexer{
    pub fn new() -> Self{
        Lexer{
            tokens : Vec::new()
        }
    }

    pub fn token_pattern(&mut self, source : &str) -> &Vec<Token>{
        for s in source.split_whitespace(){
            let token =match s {
                "+" | "-" | "*" | "/" | "=" => Op(s.to_string()),

                s if s.chars().any(|c|c.is_ascii_alphabetic())
                => Atom(AtomKind::Variable(s.to_string())),

                s if s.parse::<f64>().is_ok() => {
                    let num = s.parse::<f64>().unwrap();
                    Atom(AtomKind::Number(num))
                }
                _ => panic!("無法識別的 Token: {:?}", s),
            };
            self.tokens.push(token)
        }
        &self.tokens
    }
    pub fn print(&mut self){
        for token in &self.tokens{
            println!("{:?}", token)
        }
    }
}