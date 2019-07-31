use std::io;
use std::io::prelude::*;

mod calculator;
use calculator::parser::Parser;
use calculator::lexer::Lexer;

fn main() {
    let stdin = io::stdin();
    let mut parser = Parser::new();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let lexer = Lexer::new(&s);
        parser.reset();
        for tok in lexer {
            parser.state = parser.next(&tok);
        }
        println!("{:?}", parser.stack);
    }
}
