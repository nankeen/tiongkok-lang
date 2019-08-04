#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Integer(u64),
    Operator(char),
    End
}

pub enum ASTNode {
    BinOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    Num {
        value: u64,
    }
}
