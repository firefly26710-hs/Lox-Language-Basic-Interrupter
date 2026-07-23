use std::collections::HashMap;
use crate::lex::token::AtomKind::{Number, Variable};
use crate::lex::token::Token;

#[derive(Debug)]
pub struct Node{
    token : Token,
    left  : Option<usize>,
    right : Option<usize>
}

impl Node{
    pub fn new(new_token: Token, new_left : Option<usize>, new_right : Option<usize>) -> Self{
        Node{
            token : new_token,
            left  : new_left,
            right : new_right
        }
    }
    pub fn eval(&self, arena: &[Node], variables: &HashMap<String, f32>) -> f32 {
        match &self.token {
            Token::Atom(Number(n)) => *n as f32,
            Token::Atom(Variable(name)) => {
                *variables.get(name).expect(&format!("Undefined Variable {}", name))
            }

                // [情況 B] 單目運算子 (例如 -a)
            Token::Op('-') if self.left.is_none() => {
                let rhs_idx = self.right.expect("Unary op must have right child");
                let rhs_val = arena[rhs_idx].eval(arena, variables);
                -rhs_val
            }

                // [情況 C] 二目運算子 (例如 + - * /)
            Token::Op(op) => {
                let lhs_idx = self.left.expect("Binary op must have left child");
                let rhs_idx = self.right.expect("Binary op must have right child");

                let lhs_val = arena[lhs_idx].eval(arena, variables);
                let rhs_val = arena[rhs_idx].eval(arena, variables);

                match op {
                    '+' => lhs_val + rhs_val,
                    '-' => lhs_val - rhs_val,
                    '*' => lhs_val * rhs_val,
                    '/' => lhs_val / rhs_val,
                    _   => panic!("Bad operator {}", op),
                }
            }

            _ => panic!("Unsupported token in eval: {:?}", self.token),
        }
    }

}