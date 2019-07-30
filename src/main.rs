use std::io;
use std::io::prelude::*;

mod calculator;

fn main() {
    let stdin = io::stdin();
    let mut interpreter = calculator::Interpreter::new();
    for line in stdin.lock().lines() {
        let s = line.unwrap();
        let mut lexer = calculator::Lexer::new(&s);
        interpreter.reset();
        loop {
            let tok = lexer.next_token();
            interpreter.state = interpreter.next(&tok);
            if tok == calculator::Token::End {
                break;
            }
        }
        println!("{:?}", interpreter.stack);
    }
}
