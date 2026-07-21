pub mod lex;
pub mod par;

use crate::lex::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("100 + 1");
}
