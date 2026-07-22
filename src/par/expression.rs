use std::collections::HashMap;
use crate::lex::token::AtomKind;
use crate::lex::lexer::Lexer;
use crate::par::parser::parser_expression;

#[derive(Debug)]
pub enum Expression{
    Atom(AtomKind),
    Op(char, Vec<Expression>)
}

impl Expression {
    pub fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        lexer.scan_tokens();
        let mut token = lexer.tokens.into_iter().peekable();
        parser_expression(&mut token, 0.0)
    }
    pub fn is_assign(&self) -> Option<(String, &Expression)> {
        match self {
            Expression::Op('=', operands)if operands.len() == 2 => {
                let var_name = match &operands[0] {
                    Expression::Atom(AtomKind::Variable(c)) => c.clone(),
                    _ => return None
                };
                let rhs = &operands[1];
                Some((var_name, rhs))
            }
            _ => None,
        }
    }

    pub fn eval(&self, var_table: &mut HashMap<String, f64>) -> f64 {
        match self {
            Expression::Atom(c) => {
                match c {
                    AtomKind::Number(val) => *val,
                    AtomKind::Variable(c) =>
                        *var_table.get(c).expect(&format!("Undefined Variable {}", c)),
                }
            }


            Expression::Op(operator, operands) => {
                let lhs = operands.first().unwrap().eval(var_table);
                let rhs = operands.last().unwrap().eval(var_table);
                match operator {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    op => panic!("Bad operator {}", op)
                }
            }
        }
    }
}







