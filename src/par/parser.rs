use std::collections::HashMap;
use crate::lex::lexer::Lexer;
use crate::lex::token::{Token};
use crate::par::node::{Node, NodeKind};

pub struct Parser{
    pub nodes      : Vec<Node>,
    tokens         : Vec<Token>,
    current        : usize
}

impl Parser{
    pub fn new(input_tokens: Vec<Token>)-> Self{
        Parser{
            nodes   : Vec::new(),
            tokens  : input_tokens,
            current : 0
        }
    }
    pub fn parser_statment(&mut self)->usize{
       let state = self.peek().expect("It's an empty tokens vec");
        match state{ 
            Token::LET   => self.let_statment(),
            Token::IF    => self.if_statment(),
	    Token::ELSE  => self.else_statment(),
	    Token::WHILE => self.while_statment(),
	    Token::FOR   => self.for_statment(),
	    Token::FUN   => self.function_statment(),
	    
            _ => panic!("It's an illegal statment")
        };
	self.nodes.len() - 1
    }


    fn parser_expression(&mut self, min_bp : f32)->usize{	
        let number_token = self.advance().expect("Invalid lhs");
        let mut lhs_index: usize = match number_token {
            Token::Number(_) | Token::Identifier(_) => {
                self.nodes.push(Node::Leaf(number_token.clone()));
                self.nodes.len() - 1
            }
            Token::MINUS                 =>{
                let rhs_index = self.parser_expression(1000.0);
                self.nodes.push(Node::Binary(Token::MINUS, None, Some(rhs_index)));
                self.nodes.len() - 1
            }
	    Token::CMPNOT =>{
		let rhs_index = self.parser_expression(1000.0);
		self.nodes.push(Node::Binary(Token::CMPNOT, None, Some(rhs_index)));
		self.nodes.len() - 1
	    }
            Token::LEFTPAREN => {
                let lhs_index = self.parser_expression(0.0);
		match self.peek(){
		    Some(Token::RIGHTPAREN) => self.advance(),
		    _ => panic!("Not Include )")
		};
		lhs_index
            }
            _ => panic!("Expected atom"),
        };

        loop {
            let op_token = match self.peek().expect("Invalid Op") {
                Token::EOF        | Token::RIGHTPAREN
               |Token::RIGHTBRACE | Token::SEMICOLON => break,
		
                op_token => op_token.clone(),
            };	    
            let (l_bp, r_bp) = infix_blind_power(op_token.clone());
            if l_bp < min_bp {
                break;
            }
	    self.advance();
            let rhs_index: usize = self.parser_expression(r_bp);
            self.nodes.push(Node::Binary(op_token.clone(), Some(lhs_index), Some(rhs_index)));
            lhs_index = self.nodes.len() - 1;
        }
        lhs_index
	

    }

    
    fn let_statment(&mut self) -> usize{

	let let_token = match self.peek(){
	    Some(Token::LET) => self.advance().unwrap(),
	    _ => panic!("This is not let token")
	};
	
        let var_token = self.advance().expect("are you kidding me in var");
        let lhs_index = {
            self.nodes.push(Node::Leaf(var_token));
            self.nodes.len() - 1
        };

        match self.peek() {
	    Some(Token::EQUEL) => self.advance().unwrap(),
	    _ => panic!("Expected '=' at the middle of statement")
	};

        let rhs_index = self.parser_expression(0.0);
	
	match self.peek() {
            Some(Token::SEMICOLON) => self.advance().unwrap(),
            _ => panic!("Expected ';' at the end of statement"),
        };


        self.nodes.push(Node::Binary(let_token, Some(lhs_index), Some(rhs_index)));
	self.nodes.len() - 1
    }
    

    // > >= < <= == != 
    // if x > 0 {let b = 2 }
    fn if_statment(&mut self) -> usize{
	let if_token = match self.peek(){
	    Some(Token::IF) => self.advance().unwrap(),
	    _ =>  panic!("Expected 'if' at the middle of statement")
	};

	let condition_index = self.condition();
	
	match self.peek(){
	    Some(Token::LEFTBRACE) => self.advance().unwrap(),
	    _ => panic!("Expected Left Brace at the middle of statement")
	};
	let inner_statment_index = self.parser_statment();
	
        match self.peek(){
	    Some(Token::RIGHTBRACE) => self.advance().unwrap(),
	   _ => panic!("Expected Right Brace at the middle of statement")
	};
	self.nodes.push(Node::Binary(if_token, Some(condition_index), Some(inner_statment_index)));
	self.nodes.len() - 1
    }

    fn else_statment(&mut self)    -> usize{1}
    fn for_statment(&mut self)     -> usize{1}
    fn while_statment(&mut self)   -> usize{1}
    fn function_statment(&mut self)->usize{1}
    
    fn condition(&mut self) -> usize{
	let lhs_token = match self.peek(){
	    Some(Token::Identifier(_)) | Some(Token::Number(_)) => self.advance().unwrap(),
	    _ => panic!("Expected Atom at the middle of statement")
	};
	
	let lhs_index = {
	    self.nodes.push(Node::Leaf(lhs_token));
            self.nodes.len() - 1
	};

	let op_token = match self.peek(){
	    Some(Token::GREATER)   |Some(Token::GREATEREUQEL)|
	    Some(Token::LESSER)    |Some(Token::LESSEREQUEL)|
	    Some(Token::EQUELEQUEL)|Some(Token::NOTEQUEL)
		=> self.advance().unwrap(),//pass
	    _   => panic!("no this condiction operator")
	};

	let rhs_token = match self.peek(){
	    Some(Token::Identifier(_)) | Some(Token::Number(_)) => self.advance().unwrap(),
	    _ => panic!("Expected Atom at the middle of statment")
	};

	let rhs_index = {
	    self.nodes.push(Node::Leaf(rhs_token));
	    self.nodes.len() - 1
	};
	
	self.nodes.push(Node::Binary(op_token.clone(), Some(lhs_index), Some(rhs_index)));
	self.nodes.len() - 1
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
        for (i, node) in self.nodes.into_iter().enumerate(){
            println!("Node : {}, {:?}",i, node);
        }
    }

   pub fn eval(&self, index: usize, var_table: &mut HashMap<String, f64>) -> f64 {
        let node = &self.nodes[index];

        match &node.kind {
            NodeKind::BinaryNode { token, left, right } => {
                let left_opt = left.map(|x| x as usize);
                let right_opt = right.map(|x| x as usize);

                match (left_opt, right_opt) {
                    (None, None) => match token {
                        Token::Number(val) => *val,
                        Token::Identifier(name) => {
                            *var_table.get(name).expect(&format!("Undefined variable: {}", name))
                        }
                        _ => panic!("Expected atom"),
                    },
                    (None, Some(right_idx)) => {
                        let val = self.eval(right_idx, var_table);
                        match token {
                            Token::MINUS => -val,
                            Token::PLUS => val,
                            _ => panic!("Invalid unary operator"),
                        }
                    }
                    (Some(left_idx), Some(right_idx)) => {
                        if token == &Token::EQUEL {
                            let val = self.eval(right_idx, var_table);

                            let var_name = match &self.nodes[left_idx].kind {
                                NodeKind::BinaryNode {
                                    token: Token::Identifier(name),
                                    ..
                                } => name.clone(),
                                _ => panic!("Left side of assignment must be a variable"),
                            };

                            var_table.insert(var_name, val);
                            return val;
                        }

                        let left_val = self.eval(left_idx, var_table);
                        let right_val = self.eval(right_idx, var_table);
                        match token {
                            Token::PLUS => left_val + right_val,
                            Token::MINUS => left_val - right_val,
                            Token::MULTI => left_val * right_val,
                            Token::DIVIDE => left_val / right_val,
                            _ => panic!("Invalid binary operator"),
                        }
			
                    }
                    _ => panic!("Invalid node structure")
                }
            },
	    &NodeKind::BranchTable { .. } => todo!()
        }
   }

}




fn infix_blind_power(op : Token) -> (f32, f32){
    match op{
	Token::CMPAND | Token::CMPOR    => (0.1, 0.2),
        Token::EQUELEQUEL               => (0.3, 0.4),
	Token::GREATER| Token::LESSER   => (0.5, 0.5),
        Token::PLUS   | Token::MINUS    => (1.0, 2.0),
        Token::MULTI  | Token::DIVIDE   => (3.0, 4.0),
        _         => panic!("Unknown Operator{:?}", op)
    }
}


#[test]
fn let_parser(){
    let input = "
    let x = not(-12 > 3) and not(5 < 9);
    let z = 5;
";
    let mut lexer = Lexer::new(&input);
    lexer.scan_tokens();
    //dbg!(&lexer.tokens);
    lexer.print();
    let mut parser = Parser::new(lexer.tokens);
    parser.parser_statment();
    //dbg!(&parser.nodes);
    parser.print();

    
}

