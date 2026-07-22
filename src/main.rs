pub mod lex;
pub mod par;

use crate::lex::lexer::Lexer;
use crate::par::parser::parser_expression;
fn main() {
    let input = "114514 * 17 - 26928";
    let mut lexer = Lexer::new(&input);
    lexer.scan_tokens();
    lexer.print();
    let mut tokens = lexer.tokens.into_iter().peekable();
    let ast = parser_expression(&mut tokens, 0.0);
    let result = ast.eval();

    println!("-------------------");
    println!("Input : {}", input);
    println!("AST   : {:?}", ast);
    println!("Result: {}", result);
}
