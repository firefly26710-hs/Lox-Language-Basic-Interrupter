use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lex::token::{Token};
use crate::par::node::Node;

pub struct SyntaxTree{
    pub syntax_tree: Vec<Node>
}

impl SyntaxTree{
    pub fn new()-> Self{
        SyntaxTree{
            syntax_tree : Vec::new()
        }
    }

    pub fn parser_expression(&mut self,tokens : &mut Peekable<IntoIter<Token>>, min_bp : f32)->usize{
        let lhs_index = match tokens.next().expect("Invalid Token in lhs") {
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind), None, None));
                self.syntax_tree.len() - 1
            }

            t => panic!("bad token!!{:?}", t)
        };
        loop {
            let op = match tokens.peek().expect("Invalid Op"){
                Token::EOF => break,
                Token::Op(op) => op,
                t => panic!("bad token!!{:?}", t),
            };
            tokens.next();
            let (l_bp, r_bp) = infix_blind_power(*op);
            if l_bp < min_bp{
                break;
            }
            let rhs_index = self.parser_expression(tokens, r_bp);
        }
        lhs_index

    }
}


fn infix_blind_power(op : char) -> (f32, f32){
    match op{
        '='       => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        _         => panic!("Unknown Operator{:?}", op)
    }
}