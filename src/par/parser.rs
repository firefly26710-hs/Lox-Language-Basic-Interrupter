                Token::Op('(') =>{
            }
    };
    loop{
        };
        let (l_bp, r_bp) = infix_blind_power(op);
        if l_bp < min_bp{
            break;
        }
    }
}

fn infix_blind_power(op : char) -> (f32, f32){
    match op{
        '='       => (0.1, 0.2),
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
    }
}