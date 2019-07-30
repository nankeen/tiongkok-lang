use std::vec::Vec;

// Runs with default radix 10
// const RADIX: u32 = 10;

#[derive(Debug)]
pub enum Token {
    Integer(u32),
    Operator(char),
    End,
}

#[derive(Debug)]
pub enum State {
    Start,
    Integer,
    Add,
    End,
}

pub struct Interpreter {
    pub state: State,
    pub stack: Vec<u32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            state: State::Start,
            stack: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = State::Start;
        self.stack.clear();
    }

    pub fn next(&mut self, event: Token) -> State {
        use State::*;
        match self.state {
            Start => self.handle_start(event),
            Integer => self.handle_digit(event),
            Add => self.handle_add(event),
            End => {
                match self.stack.pop() {
                    Some(ret) => println!("{:?}", ret),
                    None => println!("nil"),
                };
                State::End
            },
            /*
            Operator => self.handle_operator(event),
            End => End,
            */
        }
    }

    fn handle_start(&mut self, event: Token) -> State {
        use Token::*;
        match event {
            Integer(d) => {
                self.stack.push(d);
                State::Integer
            },
            End => State::End,
            _ => panic!("Syntax error"),
        }
    }

    fn handle_digit(&mut self, event: Token) -> State {
        use Token::*;
        match event {
            Operator(op) => match op {
                '+' => State::Add,
                _ => panic!("Invalid operator"),
            },
            End => State::End,
            _ => panic!("Syntax error"),
        }
    }

    fn handle_add(&mut self, event: Token) -> State {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a+d);
                State::Integer
            },
            _ => panic!("Syntax error")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::discriminant;
    
    macro_rules! assert_enum_eq {
        ($a:expr,$b:expr) => {
            assert_eq!(discriminant($a), discriminant($b));
        };
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_start_state() {
        // Start should transition to digit state
        let mut interpreter = Interpreter::new();
        let mut ns = interpreter.next(Token::Integer(1));
        assert_enum_eq!(&ns, &State::Integer);

        // Test invalid transition panic
        interpreter.next(Token::Operator('+'));

        // Test digit can end
        ns = interpreter.next(Token::End);
        assert_enum_eq!(&ns, &State::End);

    }

    #[test]
    #[should_panic(expected = "Invalid operator")]
    fn test_integer_state() {
        // Integer should transition to `Add` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(Token::Integer(1));

        let mut ns = interpreter.next(Token::Operator('+'));
        assert_enum_eq!(&ns, &State::Add);

        // Test invalid operator panic
        interpreter.next(Token::Operator('>'));

        // Test digit can end
        ns = interpreter.next(Token::End);
        assert_enum_eq!(&ns, &State::End);
    }

    #[test]
    fn test_add_state() {
        // Add should transition to `Integer` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(Token::Integer(5));
        interpreter.state = interpreter.next(Token::Operator('+'));

        let ns = interpreter.next(Token::Integer(3));
        assert_enum_eq!(&ns, &State::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5+3);
    }
}
