use std::io;
use std::io::prelude::*;

mod calculator;
use calculator::parser::Parser;
use calculator::lexer::Lexer;
use calculator::interpreter;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let lexer = Lexer::new(&s);
        let mut parser = Parser::new(lexer);
        println!("{:?}", interpreter::visit(&parser.expr()));
        // println!("{:?}", interpreter::postfix_notation(&parser.expr()));
        // println!("{}", interpreter::lisp_notation(&parser.expr()));
    }
}
