use std::collections::HashMap;
use std::io;
use std::io::Write;
fn main() {
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let input = {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            buf
        };
            break;
        }
        }

