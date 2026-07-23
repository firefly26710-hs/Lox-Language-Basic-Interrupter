use crate::lex::lexer::Lexer;
use crate::par::parser::SyntaxTree;

pub mod lex;
pub mod par;

fn main() {
    let input = "1 + 1 * 1";
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();
    println!("tokens {:?}", &lexer.tokens);
    let mut syntax_tree = SyntaxTree::new();
    syntax_tree.parser_expression(&mut lexer.tokens, 0);
    println!("Expression: {}", input);
    println!("AST Arena: {:?}", syntax_tree.syntax_tree);
}