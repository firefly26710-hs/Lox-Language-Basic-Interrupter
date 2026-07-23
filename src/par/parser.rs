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

    pub fn parser_expression(&mut self, mut tokens : &mut Vec<Token>, index: usize){
        let lhs_index:usize = match tokens.get(index).expect("Invalid lhs"){
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.syntax_tree.len() - 1
            }

            t => panic!("bad token!!{:?}", t)
        };

        let op = match tokens.get(index + 1).expect("Invalid Op"){
            Token::Op(op) => op.clone(),
            t => panic!("bad token!!{:?}", t)
        };
        let rhs_index:usize = match tokens.get(index + 2).expect("Invalid rhs"){
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.syntax_tree.len() - 1
            }

            t => panic!("bad token!!{:?}", t)
        };
        self.syntax_tree.push(Node::new(Token::Op(op), Some(lhs_index), Some(rhs_index)))
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