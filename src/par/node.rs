use crate::lex::token::Token;

#[derive(Debug, Clone)]
pub enum NodeKind{
    BinaryNode{
	token : Token,
	left  : Option<usize>,
	right : Option<usize>
    },
    
    BranchTable{
	table : Vec<u16>
    }
    

}



#[derive(Debug)]
pub struct Node{
    pub kind : NodeKind
}

impl Node{
    pub fn binary(new_token: Token, new_left: Option<usize>, new_right: Option<usize>) -> Self {
        Self {
            kind: NodeKind::BinaryNode {
                token: new_token,
                left : new_left,
                right: new_right,
            },
        }
    }
        
    

   pub fn leaf(new_token: Token) -> Self {
        Self {
            kind: NodeKind::BinaryNode {
                token: new_token,
                left: None,
                right: None,
            },
        }
    }

   pub fn branch_table(table: Vec<u16>) -> Self {
        Self {
            kind: NodeKind::BranchTable {
		table
	    },
        }
    }
    
    



}
