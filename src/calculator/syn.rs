#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Integer(u64),
    Operator(char),
    End
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    BinOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    Num(i64),
}
