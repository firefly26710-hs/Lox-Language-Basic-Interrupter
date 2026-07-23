use std::iter::Peekable;
use std::vec::IntoIter;
use crate::lex::token::{Token};
use crate::par::node::Node;

pub struct SyntaxTree{
    pub(crate) syntax_tree: Vec<Node>
}

impl SyntaxTree{
    pub fn new()-> Self{
        SyntaxTree{
            syntax_tree : Vec::new()
        }
    }

    pub fn parser_expression(&mut self, tokens: &mut Peekable<IntoIter<Token>>, min_bp : f32) -> usize{
        let token =  tokens.next().expect("Expected LHS");
        let mut lhs_index:usize = match token {
            // Atom
            Token::Atom(_) =>{
                let node = Node::new(token, None, None);
                self.syntax_tree.push(node);
                self.syntax_tree.len() - 1
            }
            // unary
            Token ::Op('-')=>{
                let prefix_bp = 100.0;
                let right_idx = self.parser_expression(tokens, prefix_bp);

                let node = Node::new(token, None, Some(right_idx));
                self.syntax_tree.push(node);
                self.syntax_tree.len() - 1
            }
            // brackets
            Token::Op('(')=>{
                let inner_idx = self.parser_expression(tokens, 0.0);
                assert_eq!(tokens.next(), Some(Token::Op(')')));
                inner_idx
            }
            _ => panic!("Unexpected token: {:?}", token)
        };
        while let Some(next_token) = tokens.peek(){
            if let Token::Op(op  ) = next_token{
                let (l_bp, r_bp) = infix_blind_power(*op);

                if l_bp < min_bp{
                    break;
                }

                let op_token = tokens.next().unwrap();
                let rhs_index = self.parser_expression(tokens, r_bp);

                let node = Node::new(op_token, Some(lhs_index), Some(rhs_index));
                self.syntax_tree.push(node);
                lhs_index = self.syntax_tree.len() - 1;
            }else{
                break;
            }
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