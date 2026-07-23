use crate::par::node::Node;
use crate::lex::token::{Token};

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

        let lhs_index: usize = match token {
            Token::Atom(atom_kind) => {
                self.nodes.push(Node::new(Token::Atom(atom_kind.clone()), None, None));
                self.nodes.len() - 1
            }
            _ => panic!("Expected atom"),
        };

        loop {
            let op = match self.peek().expect("Invalid Op") {
                Token::EOF => break,
                Token::Op(op) => op.clone(),
                t => panic!("bad token!!{:?}", t)
            };
            self.advance();
            let (l_bp, r_bp) = infix_blind_power(op);
            if l_bp < min_bp {
                break;
            }
            let rhs_index: usize = self.parser_expression(r_bp);
            self.nodes.push(Node::new(Token::Op(op), Some(lhs_index), Some(rhs_index)))
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


}


fn infix_blind_power(op : char) -> (f32, f32){
    match op{
        '='       => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        _         => panic!("Unknown Operator{:?}", op)
    }
}

