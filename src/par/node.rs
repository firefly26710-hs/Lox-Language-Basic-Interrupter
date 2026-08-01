use crate::lex::token::Token;

#[derive(Debug)]
pub struct Node{
    pub token : Token,
    pub left  : Option<usize>,
    pub right : Option<usize>
}

impl Node{
    pub fn new(new_token: Token, new_left : Option<usize>, new_right : Option<usize>) -> Self{
        Node{
            token : new_token,
            left  : new_left,
            right : new_right
        }
    }


}