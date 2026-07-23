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

    pub fn parser_expression(&mut self, tokens : &mut Vec<Token>, index: usize, min_bp : f32)->usize {

        let lhs_index: usize = match tokens.get(index).expect("Invalid lhs") {
            Token::Atom(atom_kind) => {
                self.syntax_tree.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.syntax_tree.len() - 1
            }

            t => panic!("bad token!!{:?}", t)
        };

        loop {
            let op = match tokens.get(index + 1).expect("Invalid Op") {
                Token::EOF => break,
                Token::Op(op) => op.clone(),
                t => panic!("bad token!!{:?}", t)
            };
            let (l_bp, r_bp) = infix_blind_power(op);
            if l_bp < min_bp {
                break;
            }
            let rhs_index: usize = self.parser_expression(tokens, index + 2, r_bp);
            self.syntax_tree.push(Node::new(Token::Op(op), Some(lhs_index), Some(rhs_index)))
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