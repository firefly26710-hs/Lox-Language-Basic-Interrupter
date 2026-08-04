use std::collections::HashMap;
use crate::lex::token::Token::{KeyWord, Op};
use crate::lex::token::KeyWordKind;
use crate::lex::token::OpKind;
use crate::par::node::Node;
use crate::lex::token::{AtomKind, Token};

pub struct SyntaxTree{
    pub nodes      : Vec<Node>,
    tokens         : Vec<Token>,
    current        : usize
}

impl SyntaxTree{
    pub fn new(input_tokens: Vec<Token>)-> Self{
        SyntaxTree{
            nodes   : Vec::new(),
            tokens  : input_tokens,
            current : 0
        }
    }
    pub fn parser_statment(&mut self){
       let state = self.advance().expect("wtf dude");
        match state{
            KeyWord(KeyWordKind::LET) => self.variable_statment(),
            _ => panic!("are you crazzy")
        }
    }


    fn parser_expression(&mut self, min_bp : f32)->usize {

        let token = self.advance().expect("Invalid lhs");

        let mut lhs_index: usize = match token {
            Token::Atom(atom_kind) => {
                self.nodes.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.nodes.len() - 1
            }
            Op(OpKind::MINUS) =>{
                let rhs_index = self.parser_expression(1000.0);
                self.nodes.push(Node::new(Op(OpKind::MINUS), None, Some(rhs_index)));
                self.nodes.len() - 1
            }
            Op(OpKind::LEFTPAREN) => {
                let lhs_index = self.parser_expression(0.0);

                match self.advance() {
                    Some(Op(OpKind::RIGHTPAREN)) => lhs_index,
                    _ => panic!("Expected  )"),
                }
            }

            _ => panic!("Expected atom"),
        };

        loop {
            let op = match self.peek().expect("Invalid Op") {
                Token::EOF     => break,
                Op(OpKind::RIGHTPAREN) => break,
                Op(op) => op.clone(),
                t => panic!("bad token!!{:?}", t)
            };
            let (l_bp, r_bp) = infix_blind_power(op.clone());
            if l_bp < min_bp {
                break;
            }
            self.advance();

            let rhs_index: usize = self.parser_expression(r_bp);
            self.nodes.push(Node::new(Op(op.clone()), Some(lhs_index), Some(rhs_index)));
            lhs_index = self.nodes.len() - 1;
        }

        lhs_index

    }
    fn variable_statment(&mut self){
        let var_token = self.advance().expect("are you kidding me in var");
        let lhs_index = {
            self.nodes.push(Node::new(var_token, None, None));
            self.nodes.len() - 1
        };

        let eq_token = if matches!(self.peek(), Some(Op(OpKind::EQUEL))) {
            self.advance().expect("are you kidding me in eq")
        } else {
            panic!("wtf, this is not a eq")
        };

        let rhs_index = self.parser_expression(0.0);

        self.nodes.push(Node::new(eq_token, Some(lhs_index), Some(rhs_index)));
    }
    fn advance(&mut self) -> Option<Token>{
        let token = self.tokens.get(self.current).cloned();
        self.current += 1;
        token
    }

    fn peek(&self) -> Option<&Token>{
        self.tokens.get(self.current)
    }

    pub fn print(self){
        for node in self.nodes{
            println!("{:?}", node);
        }
    }

    pub fn eval(&self, index: usize, var_table: &mut HashMap<String, f64>) -> f64 {
        let node = &self.nodes[index];

        match (node.left, node.right) {

            //This is a variable
            (None, None) => {
                match &node.token {
                    Token::Atom(AtomKind::Number(val)) => *val,
                    Token::Atom(AtomKind::Idenitifer(name)) => {
                        *var_table.get(name).expect(&format!("Undefined variable: {}", name))
                    }
                    _ => panic!("Expected atom"),
                }
            }

            // This is an unary
            (None, Some(right_idx)) => {
                let val = self.eval(right_idx, var_table);
                let op = &node.token;
                match op {
                    Op(OpKind::MINUS) => -val,
                    Op(OpKind::PLUS) => val,
                    _ => panic!("Invalid unary operator"),
                }
            }

            // This is a expression
            (Some(left_idx), Some(right_idx)) => {
                let op = &node.token;
                if *op == Op(OpKind::EQUEL){
                    let val = self.eval(right_idx, var_table);

                    let var_name = match &self.nodes[left_idx].token {
                        Token::Atom(AtomKind::Idenitifer(name)) => name.clone(),
                        _ => panic!("Left side of assignment must be a variable"),
                    };

                    var_table.insert(var_name, val);
                    return val;
                }

                let left_val = self.eval(left_idx, var_table);
                let right_val = self.eval(right_idx, var_table);
                match node.token {
                    Op(OpKind::PLUS) => left_val + right_val,
                    Op(OpKind::MINUS) => left_val - right_val,
                    Op(OpKind::MULTI) => left_val * right_val,
                    Op(OpKind::DIVIDE) => left_val / right_val,
                    _ => panic!("Invalid binary operator"),
                }
            }


            _ => panic!("Invalid node structure"),
        }
    }




}




fn infix_blind_power(op : OpKind) -> (f32, f32){
    match op{
        OpKind::EQUEL                  => (0.1, 0.2),
        OpKind::PLUS  | OpKind::MINUS  => (1.0, 1.1),
        OpKind::MULTI | OpKind::DIVIDE => (2.0, 2.1),
        _         => panic!("Unknown Operator{:?}", op)
    }
}

