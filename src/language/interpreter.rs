use super::syn::{ASTNode, Token};

pub fn visit(node: &ASTNode) -> i64 {
    // Generic visit function that maps operations to their functions
    match node {
        ASTNode::BinOp{left: _ , op: _, right: _} => visit_binop(node),
        ASTNode::UnaryOp{op: _, expr: _} => visit_unaryop(node),
        ASTNode::Num(i) => *i,
        _ => panic!("Not implemented"),
    }
}

fn visit_binop(node: &ASTNode) -> i64 {
    if let ASTNode::BinOp{left, op, right} = node {
        return match op {
            Token::Add => visit(left) + visit(right),
            Token::Sub => visit(left) - visit(right),
            Token::Mul => visit(left) * visit(right),
            Token::Div => visit(left) / visit(right),
            _ => panic!("Invalid syntax found in AST"),
        };
    }
    panic!("Expected BinOp!");
}

fn visit_unaryop(node: &ASTNode) -> i64 {
    if let ASTNode::UnaryOp{op, expr} = node {
        return match op {
            Token::Add => visit(expr),
            Token::Sub => -visit(expr),
            _ => panic!("Invalid syntax found in AST"),
        };
    }
    panic!("Expected BinOp!");
}
