use crate::lex::lexer::Lexer;
use crate::lex::token::{AtomKind, Token};
use crate::lex::token::AtomKind::{Number, Variable};
use crate::par::expression::Expression;

struct Parser{
    tokens   : Vec<Token>,
    position : usize
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            tokens: lexer.tokens,
            position: 0
        }
    }

    pub fn parser_expression(&mut self, min_bp : f32) -> Expression {
        let mut lhs = match self.peek() {
            Some(it) => {
                match it {
                    Token::Atom(Number(it)) => Expression::Atom(Number(*it)),
                    Token::Atom(Variable(it)) => Expression::Atom(Variable(it.clone())),
                    Token::Op('(') =>{
                        let lhs = self.parser_expression(0.0);
                        match lhs{
                            
                        }
                    },
                    _ => todo!()
                }
            },
            None => panic!("Expected Token")
        };



    }

    fn peek(&mut self) -> Option<&Token>{
        self.tokens.get(self.position)
    }
}

fn infix_blind_power(op : char) -> (f32, f32){
    match op{
        '='       => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        _         => panic!("Unknown Operator")
    }
}