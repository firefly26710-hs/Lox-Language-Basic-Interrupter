use std::collections::HashMap;
use crate::lex::lexer::Lexer;
use crate::par::parser::SyntaxTree;

pub mod lex;
pub mod par;

fn main() {
    let input = "1 + 2 + 4";

    let mut lexer = Lexer::new(input);
    lexer.scan_tokens();
    println!("tokens {:?}", &lexer.tokens);
    let mut tokens = lexer.tokens.into_iter().peekable();
    let mut syntax_tree = SyntaxTree::new();
    let root_index = syntax_tree.parser_expression(&mut tokens, 0.0);
    let mut variables = HashMap::new();
    variables.insert("x".to_string(), 10.0);

    let root_node = &syntax_tree.syntax_tree[root_index];
    let result = root_node.eval(&syntax_tree.syntax_tree, &variables);


    println!("Expression: {}", input);
    println!("AST Arena: {:?}", syntax_tree.syntax_tree);
    println!("Result: {}", result);
}