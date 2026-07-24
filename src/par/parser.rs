use std::collections::HashMap;
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

    pub fn parser_expression(&mut self, min_bp : f32)->usize {

        let token = self.advance().expect("Invalid lhs");

        let mut lhs_index: usize = match token {
            Token::Atom(atom_kind) => {
                self.nodes.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.nodes.len() - 1
            }
            Token::Op('-') =>{
                let rhs_index = self.parser_expression(1000.0);
                self.nodes.push(Node::new(Token::Op('-'), None, Some(rhs_index)));
                self.nodes.len() - 1
            }
            Token::Op('(') => {
                let lhs_index = self.parser_expression(0.0);

                match self.advance() {
                    Some(Token::Op(')')) => lhs_index,
                    _ => panic!("Expected  )"),
                }
            }

            _ => panic!("Expected atom"),
        };

        loop {
            let op = match self.peek().expect("Invalid Op") {
                Token::EOF     => break,
                Token::Op(')') => break,
                Token::Op(op) => op.clone(),
                t => panic!("bad token!!{:?}", t)
            };
            let (l_bp, r_bp) = infix_blind_power(op);
            if l_bp < min_bp {
                break;
            }
            self.advance();

            let rhs_index: usize = self.parser_expression(r_bp);
            self.nodes.push(Node::new(Token::Op(op), Some(lhs_index), Some(rhs_index)));
            lhs_index = self.nodes.len() - 1;
        }

        lhs_index

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
            (None, None) => {
                match &node.token {
                    Token::Atom(AtomKind::Number(val)) => *val,
                    Token::Atom(AtomKind::Variable(name)) => {
                        *var_table.get(name).expect(&format!("Undefined variable: {}", name))
                    }
                    _ => panic!("Expected atom"),
                }
            }

            (None, Some(right_idx)) => {
                let val = self.eval(right_idx, var_table);
                match node.token {
                    Token::Op('-') => -val,
                    Token::Op('+') => val,
                    _ => panic!("Invalid unary operator"),
                }
            }

            (Some(left_idx), Some(right_idx)) => {
                if let Token::Op('=') = node.token {
                    let val = self.eval(right_idx, var_table);

                    let var_name = match &self.nodes[left_idx].token {
                        Token::Atom(AtomKind::Variable(name)) => name.clone(),
                        _ => panic!("Left side of assignment must be a variable"),
                    };

                    var_table.insert(var_name, val);
                    return val;
                }

                let left_val = self.eval(left_idx, var_table);
                let right_val = self.eval(right_idx, var_table);
                match node.token {
                    Token::Op('+') => left_val + right_val,
                    Token::Op('-') => left_val - right_val,
                    Token::Op('*') => left_val * right_val,
                    Token::Op('/') => left_val / right_val,
                    _ => panic!("Invalid binary operator"),
                }
            }


            _ => panic!("Invalid node structure"),
        }
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

