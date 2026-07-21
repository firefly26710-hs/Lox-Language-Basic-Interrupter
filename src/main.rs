pub mod lex;
pub mod par;

use crate::lex::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new();
    lexer.token_pattern("1 + 2 = 5");
    lexer.print()
}
