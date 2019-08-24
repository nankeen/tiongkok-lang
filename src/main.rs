use std::io;
use std::io::prelude::*;

mod language;
use language::parser::Parser;
use language::lexer::Lexer;
use language::interpreter;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let lexer = Lexer::new(s.chars());
        let mut parser = Parser::new(lexer);
        println!("{:?}", interpreter::visit(&parser.expr()));
        // println!("{:?}", interpreter::postfix_notation(&parser.expr()));
        // println!("{}", interpreter::lisp_notation(&parser.expr()));
    }
}
