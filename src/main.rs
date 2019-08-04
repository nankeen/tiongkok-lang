use std::io;
use std::io::prelude::*;

mod calculator;
use calculator::parser::Parser;
use calculator::lexer::Lexer;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let lexer = Lexer::new(&s);
        // let mut parser = Parser::new(lexer);
        // println!("{:?}", parser.expr());
    }
}
