use std::mem::discriminant;
use std::vec::Vec;

// Runs with default radix 10
const RADIX: u32 = 10;

#[derive(Debug)]
pub enum Token {
    Integer(u32),
    Operator(char),
    End,
}

#[derive(Debug)]
pub enum InterpreterFSM {
    Start,
    Integer,
    Operator,
    End,
}

impl InterpreterFSM {
    pub fn next(self, event: Token) -> InterpreterFSM {
        match self {
            InterpreterFSM::Start => handleStart(event);
        }
    }

    fn handleStart(event: Token) -> InterpreterFSM {
        match event {
            Token::Integer(i) => InterpreterFSM::Integer,
        }
    }
}

pub struct Interpreter {
    token_iterator: std::str::Chars<'static>,
    current_token: Token,
    stack: Vec<u32>,
}

impl Interpreter {
    pub fn new(program: &'static str) -> Interpreter {
        Interpreter {
            token_iterator: program.chars(),
            current_token: Token::End,
            stack: Vec::new(),
        }
    }

    fn eat(&mut self, token_type: &Token) {
        let t = self.next_token();
        if discriminant(&t) == discriminant(token_type) {
            self.current_token = t;
        } else {
            panic!("Unexpected token type");
        }
    }

    pub fn next_token(&mut self) -> Token {
        let n = self.token_iterator.next();
        match n {
            Some(c) => match c {
                c if c.is_digit(RADIX) => {
                    Token::Integer(c.to_digit(RADIX).unwrap())
                },
                '+' => Token::Operator(c),
                _ => panic!("Unknown token"),
            },
            None => Token::End,
        }
    }

    pub fn eval(&mut self) -> u32 {
        self.current_token = self.next_token();
        loop {
            match self.current_token {
                Token::Integer(i) => {
                    self.eat(&Token::Operator('+'));
                    self.stack.push(i);
                },
                Token::Operator(_) => self.eat(&Token::Integer(0)),
                Token::End => break,
            }
        }

        return self.stack.iter().sum();
    }
}
