use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lex::token::{Token};
use crate::par::expression::Expression;
use crate::par::node::Node;

struct SyntaxTree{
    syntax_tree: Vec<Node>
}

impl SyntaxTree{
    fn new()-> Self{
        SyntaxTree{
            syntax_tree : Vec::new()
        }
    }

    pub fn parser_expression(tokens: &mut Peekable<IntoIter<Token>>, min_bp : f32) -> Expression{
        




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