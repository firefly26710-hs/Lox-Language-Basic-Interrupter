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

    pub fn parser_expression(&mut self,tokens : &mut Peekable<IntoIter<Token>>, min_bp : f32){
        let lhs_index = match tokens.next().expect("Invalid Token in lhs") {
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind), None, None));
                self.syntax_tree.len() - 1
            }

            t => panic!("bad token!!{:?}", t)
        };

        let op = match tokens.next().expect("Invalid Op"){
            Token::EOF => return,
            Token::Op(op) => op,
                t => panic!("bad token!!{:?}", t),
        };

        let rhs_index = match tokens.next().expect("Invalid Token in lhs") {
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind), None, None));
                self.syntax_tree.len() - 1 }

            t => panic!("bad token!!{:?}", t)
        };

        self.syntax_tree.push(Node::new(Token::Op(op), Some(lhs_index), Some(rhs_index)));

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