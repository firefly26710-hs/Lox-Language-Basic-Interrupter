use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::lex::lexer::Lexer;
use crate::par::parser::SyntaxTree;

pub mod lex;
pub mod par;

fn main() {
    let mut var_table:HashMap<String, f64> = HashMap::new();
    let mut buf = String::with_capacity(64);
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let input = &buf;

        if input.trim() == "exit"{
            break;
        }
        let mut lexer = Lexer::new(&input);
        lexer.scan_tokens();
        println!("tokens {:?}", &lexer.tokens);
        let mut tree = SyntaxTree::new(lexer.tokens);
        tree.parser_statment();
        let start_index = tree.nodes.len() - 1;
        let result = tree.eval(start_index, &mut var_table);
        tree.print();
        println!("Result : {}", result);
    }
}