use std::vec::Vec;

// Runs with default radix 10
// const RADIX: u32 = 10;

/* Calculator interpreter
 * ======================
 * The interpreter is a finite state machine
 * that uses tokens for transitioning.
 * The grammar must be such:
 * [number][operator][number][operator]....
 *
 * The lexer is also a finite state machine
 * I'm really tired of looking at that mess
 * It does not perform grammar checking
 */

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Operator(char),
    End,
}

#[derive(Debug)]
pub enum InterpreterState {
    Start,
    Integer,
    Add,
    Subtract,
    Multiply,
    Divide,
    End,
}

pub struct Interpreter {
    pub state: InterpreterState,
    pub stack: Vec<i32>,
}

impl Interpreter {
    // Initializes a new interpreter with the start state
    pub fn new() -> Interpreter {
        Interpreter {
            state: InterpreterState::Start,
            stack: Vec::new(),
        }
    }

    // Resets interpreter internals
    pub fn reset(&mut self) {
        self.state = InterpreterState::Start;
        self.stack.clear();
    }

    // Given a token (event), transition to a new state
    pub fn next(&mut self, event: &Token) -> InterpreterState {
        use InterpreterState::*;
        // Match current state with handler functions
        match self.state {
            Start => self.handle_start(event),
            Integer => self.handle_integer(event),
            Add => self.handle_add(event),
            Subtract => self.handle_subtract(event),
            Multiply => self.handle_multiply(event),
            Divide => self.handle_divide(event),
            End => {
                // Execution of expression ended, print the results
                match self.stack.pop() {
                    Some(ret) => println!("{:?}", ret),
                    None => println!("nil"),
                };
                InterpreterState::End
            },
        }
    }

    // Start state allows transition to `Integer` and `End` state
    fn handle_start(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Integer(d) => {
                self.stack.push(*d);
                InterpreterState::Integer
            },
            End => InterpreterState::End,
            _ => panic!("Syntax error"),
        }
    }

    // Integer state allows transition to `Add` and `End` state
    fn handle_integer(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Operator(op) => match op {
                '+' => InterpreterState::Add,
                '-' => InterpreterState::Subtract,
                '*' => InterpreterState::Multiply,
                '/' => InterpreterState::Divide,
                _ => panic!("Invalid operator"),
            },
            End => InterpreterState::End,
            _ => panic!("Syntax error"),
        }
    }

    // Add state allows transition to `Integer` state
    fn handle_add(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a+d);
                InterpreterState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Subtract state allows transition to `Integer` state
    fn handle_subtract(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a-d);
                InterpreterState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Multiply state allows transition to `Integer` state
    fn handle_multiply(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a*d);
                InterpreterState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Divide state allows transition to `Integer` state
    fn handle_divide(&mut self, event: &Token) -> InterpreterState {
        use Token::*;
        match event {
            Integer(d) if *d == 0 => panic!("Unable to divide by zero"),
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a/d);
                InterpreterState::Integer
            },
            _ => panic!("Syntax error")
        }
    }
}

enum LexerState {
    Digit,
    Operator,
    End,
}

pub struct Lexer<'a> {
    state: LexerState,
    iter: std::str::Chars<'a>,
    stack: Vec<char>,
}

impl Lexer<'_> {
    pub fn new(s: &str) -> Lexer {
        // Stack contains '0' to make starting with operators a valid expression
        Lexer {
            state: LexerState::Digit,
            iter: s.chars(),
            stack: vec!['0'],
        }
    }

    pub fn next_token(&mut self) -> Token {
        // TODO: Fix this crap, there must a better pattern
        use LexerState::*;
        loop {
            match self.iter.next() {
                Some(c) if c.is_digit(10) => match self.state {
                    Digit => {
                        self.stack.push(c);
                    },
                    Operator => {
                        self.state = Digit;
                        let tok = Token::Operator(self.stack.pop().unwrap());
                        self.stack.push(c);
                        return tok;
                    },
                    End => return Token::End,
                },
                Some(c) if c == ' ' => {}, // Skip white space
                Some(c) => match self.state {
                    Digit => {
                        self.state = Operator;
                        let tok = Token::Integer(self.stack.iter().collect::<String>().parse().unwrap());
                        self.stack.clear();
                        self.stack.push(c);
                        return tok;
                    },
                    Operator => {
                        let tok = Token::Operator(self.stack.pop().unwrap());
                        self.stack.push(c);
                        return tok;
                    },
                    End => return Token::End,
                }
                None => match self.state {
                    Digit => {
                        self.state = End;
                        let tok = Token::Integer(self.stack.iter().collect::<String>().parse().unwrap());
                        self.stack.clear();
                        return tok;
                    },
                    Operator => {
                        self.state = End;
                        let tok = Token::Operator(self.stack.pop().unwrap());
                        return tok;
                    },
                    End => return Token::End,
                }
            }
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
        let mut ns = interpreter.next(&Token::Integer(1));
        assert_enum_eq!(&ns, &InterpreterState::Integer);

        // Test invalid transition panic
        interpreter.next(&Token::Operator('+'));

        // Test digit can end
        ns = interpreter.next(&Token::End);
        assert_enum_eq!(&ns, &InterpreterState::End);

    }

    #[test]
    #[should_panic(expected = "Invalid operator")]
    fn test_integer_state() {
        // Integer should transition to `Add` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(&Token::Integer(1));

        let mut ns = interpreter.next(&Token::Operator('+'));
        assert_enum_eq!(&ns, &InterpreterState::Add);

        // Test invalid operator panic
        interpreter.next(&Token::Operator('>'));

        // Test digit can end
        ns = interpreter.next(&Token::End);
        assert_enum_eq!(&ns, &InterpreterState::End);
    }

    #[test]
    fn test_add_state() {
        // Add should transition to `Integer` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('+'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &InterpreterState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5+3);
    }

    #[test]
    fn test_subtract_state() {
        // Subtract should transition to `Integer` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('-'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &InterpreterState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5-3);
    }

    #[test]
    fn test_multiply_state() {
        // Multiply should transition to `Integer` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('*'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &InterpreterState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5*3);
    }

    #[test]
    #[should_panic(expected = "Unable to divide by zero")]
    fn test_divide_state() {
        // Multiply should transition to `Integer` state
        let mut interpreter = Interpreter::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('/'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &InterpreterState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5/3);

        interpreter.reset();
        interpreter.state = interpreter.next(&Token::Integer(1));
        interpreter.state = interpreter.next(&Token::Operator('/'));
        interpreter.state = interpreter.next(&Token::Integer(0));
    }

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("5+123");
        assert_eq!(lexer.next_token(), Token::Integer(5));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(123));
        assert_eq!(lexer.next_token(), Token::End);

        lexer = Lexer::new("115+123");
        assert_eq!(lexer.next_token(), Token::Integer(115));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(123));
        assert_eq!(lexer.next_token(), Token::End);

        lexer = Lexer::new("115++123");
        assert_eq!(lexer.next_token(), Token::Integer(115));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(123));
        assert_eq!(lexer.next_token(), Token::End);

        lexer = Lexer::new("115+123+1");
        assert_eq!(lexer.next_token(), Token::Integer(115));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(123));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(1));
        assert_eq!(lexer.next_token(), Token::End);

        lexer = Lexer::new("-115+123+1");
        assert_eq!(lexer.next_token(), Token::Integer(0));
        assert_eq!(lexer.next_token(), Token::Operator('-'));
        assert_eq!(lexer.next_token(), Token::Integer(115));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(123));
        assert_eq!(lexer.next_token(), Token::Operator('+'));
        assert_eq!(lexer.next_token(), Token::Integer(1));
        assert_eq!(lexer.next_token(), Token::End);
    }
}
