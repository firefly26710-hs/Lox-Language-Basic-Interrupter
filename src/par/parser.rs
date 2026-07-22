use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lex::token::{Token};
use crate::lex::token::AtomKind::{Number, Variable};
use crate::par::expression::Expression;
pub fn parser_expression(tokens: &mut Peekable<IntoIter<Token>>, min_bp : f32) -> Expression{
    let mut lhs = match tokens.next(){
        Some(it) => {
            match it{
                Token::Atom(Number(it)) => Expression::Atom(Number(it)),
                Token::Atom(Variable(it)) => Expression::Atom(Variable(it.clone())),
                Token::Op('(') =>{
                    let lhs = parser_expression(tokens, 0.0);
                    assert_eq!(tokens.next(), Some(Token::Op(')')) );
                    lhs
                },
                _ => panic!("Expected Token for LHS")
            }
        },
        None => panic!("No this Token")
    };
    loop{
        let op = match tokens.peek(){
            Some(op)=>{
                match op{
                    Token::EOF  | Token::Op(')') => break,
                    Token::Op(op)        => *op,
                    _                            => panic!("bad token")
                }
            }
            None => panic!("Expected Operator")
        };
        let (l_bp, r_bp) = infix_blind_power(op);
        if l_bp < min_bp{
            break;
        }
        tokens.next();
        let rhs = parser_expression(tokens, r_bp);
        lhs = Expression::Op(op, vec![lhs, rhs])
    }
    lhs
}

fn infix_blind_power(op : char) -> (f32, f32){
    match op{
        '='       => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        _         => panic!("Unknown Operator")
    }
}