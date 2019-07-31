use std::vec::Vec;
use super::token::Token;

#[derive(Debug)]
pub enum ParserState {
    Start,
    Integer,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Parser {
    pub state: ParserState,
    pub stack: Vec<i32>,
}

impl Parser {
    // Initializes a new interpreter with the start state
    pub fn new() -> Parser {
        Parser {
            state: ParserState::Start,
            stack: Vec::new(),
        }
    }

    // Resets interpreter internals
    pub fn reset(&mut self) {
        self.state = ParserState::Start;
        self.stack.clear();
    }

    // Given a token (event), transition to a new state
    pub fn next(&mut self, event: &Token) -> ParserState {
        use ParserState::*;
        // Match current state with handler functions
        match self.state {
            Start => self.handle_start(event),
            Integer => self.handle_integer(event),
            Add => self.handle_add(event),
            Subtract => self.handle_subtract(event),
            Multiply => self.handle_multiply(event),
            Divide => self.handle_divide(event),
        }
    }

    // Start state allows transition to `Integer` and `End` state
    fn handle_start(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Integer(d) => {
                self.stack.push(*d);
                ParserState::Integer
            },
            _ => panic!("Syntax error"),
        }
    }

    // Integer state allows transition to `Add` and `End` state
    fn handle_integer(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Operator(op) => match op {
                '+' => ParserState::Add,
                '-' => ParserState::Subtract,
                '*' => ParserState::Multiply,
                '/' => ParserState::Divide,
                _ => panic!("Invalid operator"),
            },
            _ => panic!("Syntax error"),
        }
    }

    // Add state allows transition to `Integer` state
    fn handle_add(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a+d);
                ParserState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Subtract state allows transition to `Integer` state
    fn handle_subtract(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a-d);
                ParserState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Multiply state allows transition to `Integer` state
    fn handle_multiply(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a*d);
                ParserState::Integer
            },
            _ => panic!("Syntax error")
        }
    }

    // Divide state allows transition to `Integer` state
    fn handle_divide(&mut self, event: &Token) -> ParserState {
        use Token::*;
        match event {
            Integer(d) if *d == 0 => panic!("Unable to divide by zero"),
            Integer(d) => {
                let a = self.stack.pop().unwrap();
                self.stack.push(a/d);
                ParserState::Integer
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
        let mut interpreter = Parser::new();
        let mut ns = interpreter.next(&Token::Integer(1));
        assert_enum_eq!(&ns, &ParserState::Integer);

        // Test invalid transition panic
        interpreter.next(&Token::Operator('+'));

        // Test digit can end
        ns = interpreter.next(&Token::End);
        assert_enum_eq!(&ns, &ParserState::End);

    }

    #[test]
    #[should_panic(expected = "Invalid operator")]
    fn test_integer_state() {
        // Integer should transition to `Add` state
        let mut interpreter = Parser::new();
        interpreter.state = interpreter.next(&Token::Integer(1));

        let mut ns = interpreter.next(&Token::Operator('+'));
        assert_enum_eq!(&ns, &ParserState::Add);

        // Test invalid operator panic
        interpreter.next(&Token::Operator('>'));

        // Test digit can end
        ns = interpreter.next(&Token::End);
        assert_enum_eq!(&ns, &ParserState::End);
    }

    #[test]
    fn test_add_state() {
        // Add should transition to `Integer` state
        let mut interpreter = Parser::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('+'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &ParserState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5+3);
    }

    #[test]
    fn test_subtract_state() {
        // Subtract should transition to `Integer` state
        let mut interpreter = Parser::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('-'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &ParserState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5-3);
    }

    #[test]
    fn test_multiply_state() {
        // Multiply should transition to `Integer` state
        let mut interpreter = Parser::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('*'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &ParserState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5*3);
    }

    #[test]
    #[should_panic(expected = "Unable to divide by zero")]
    fn test_divide_state() {
        // Multiply should transition to `Integer` state
        let mut interpreter = Parser::new();
        interpreter.state = interpreter.next(&Token::Integer(5));
        interpreter.state = interpreter.next(&Token::Operator('/'));

        let ns = interpreter.next(&Token::Integer(3));
        assert_enum_eq!(&ns, &ParserState::Integer);
        assert_eq!(interpreter.stack.pop().unwrap(), 5/3);

        interpreter.reset();
        interpreter.state = interpreter.next(&Token::Integer(1));
        interpreter.state = interpreter.next(&Token::Operator('/'));
        interpreter.state = interpreter.next(&Token::Integer(0));
    }
}
