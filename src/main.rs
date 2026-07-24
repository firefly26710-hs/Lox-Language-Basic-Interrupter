use crate::lex::lexer::Lexer;
use crate::par::parser::SyntaxTree;

pub mod lex;
pub mod par;

fn main() {
    let input = "1 + 1 + 1";
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();
    println!("tokens {:?}", &lexer.tokens);
    let mut syntax_tree = SyntaxTree::new(lexer.tokens);
    syntax_tree.parser_expression(0.0);
    println!("Expression: {}", input);
    syntax_tree.print();
}