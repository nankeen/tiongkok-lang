use std::io;
use std::io::prelude::*;

mod calculator;

fn main() {
    let stdin = io::stdin();
    let mut interpreter = calculator::Interpreter::new();
    for line in stdin.lock().lines() {
        interpreter.reset();
        for e in line.unwrap().chars() {
            let token = match e {
                e if e.is_digit(10) => {
                    calculator::Token::Integer(e.to_digit(10).unwrap())
                },
                '+' => calculator::Token::Operator('+'),
                _ => panic!("Syntax error"),
            };
            interpreter.state = interpreter.next(token);
        }
        interpreter.next(calculator::Token::End);
        println!("{:?}", interpreter.stack);
    }
}
