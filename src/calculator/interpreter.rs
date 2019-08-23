use std::vec::Vec;
use super::syn::{ASTNode, Token};

pub fn visit(node: &ASTNode) -> i64 {
    match node {
        ASTNode::BinOp{left, op, right} => {
            match op {
                Token::Operator('+') => visit(left) + visit(right),
                Token::Operator('-') => visit(left) - visit(right),
                Token::Operator('*') => visit(left) * visit(right),
                Token::Operator('/') => visit(left) / visit(right),
                _ => panic!("Invalid syntax found in AST"),
            }
        },
        ASTNode::UnaryOp{op, expr} => {
            match op {
                Token::Operator('+') => visit(expr),
                Token::Operator('-') => -visit(expr),
                _ => panic!("Invalid syntax found in AST"),
            }
        }
        ASTNode::Num(i) => *i,
    }
}

pub fn postfix_notation(node: &ASTNode) -> String {
    match node {
        ASTNode::BinOp{left, op, right} => {
            match op {
                Token::Operator(op) => format!("{} {} {}", postfix_notation(left), postfix_notation(right), op),
                _ => panic!("Invalid syntax found in AST"),
            }
        },
        ASTNode::Num(i) => i.to_string(),
        _ => panic!("Not implemented"),
    }
}

pub fn lisp_notation(node: &ASTNode) -> String {
    match node {
        ASTNode::BinOp{left, op, right} => {
            match op {
                Token::Operator(op) => format!("({} {} {})", op, lisp_notation(left), lisp_notation(right)),
                _ => panic!("Invalid syntax found in AST"),
            }
        },
        ASTNode::Num(i) => i.to_string(),
        _ => panic!("Not implemented"),
    }
}
