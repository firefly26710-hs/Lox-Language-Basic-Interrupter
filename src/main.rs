pub mod lex;
pub mod par;

use std::collections::HashMap;
use std::io;
use std::io::Write;
use crate::par::expression::Expression;
fn main() {
    let mut variable: HashMap<String, f64> = HashMap::new();
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            buf
        };
        
        if input.trim() == "clear"{
            break;
        }
        let expr = Expression::from_str(&input);
        if let Some((var_name, lhs)) = expr.is_assign(){
            let value = lhs.eval(&mut variable);
            variable.insert(var_name, value);
            continue;
        }
        let value = expr.eval(&mut variable);
        println!("{}", value)
    }

}
