#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Integer(u64),
    Id(String),

    // +, -, *, /, (, ), {, }, :=
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
    LCBrac,
    RCBrac, 
    Assign,

    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    BinOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    UnaryOp {
        op: Token,
        expr: Box<ASTNode>,
    },
    Num(i64),
}
