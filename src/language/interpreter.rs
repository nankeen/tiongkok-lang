use super::syn::{ASTNode, Token};

pub fn visit(node: &ASTNode) -> i64 {
    match node {
        ASTNode::BinOp{left, op, right} => {
            match op {
                Token::Add => visit(left) + visit(right),
                Token::Sub => visit(left) - visit(right),
                Token::Mul => visit(left) * visit(right),
                Token::Div => visit(left) / visit(right),
                _ => panic!("Invalid syntax found in AST"),
            }
        },
        ASTNode::UnaryOp{op, expr} => {
            match op {
                Token::Add => visit(expr),
                Token::Sub => -visit(expr),
                _ => panic!("Invalid syntax found in AST"),
            }
        }
        ASTNode::Num(i) => *i,
    }
}
