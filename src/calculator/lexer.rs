use std::vec::Vec;
use crate::calculator::token::Token;


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
            stack: Vec::new(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
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
                        return Some(tok);
                    },
                    End => return None,
                },
                Some(c) if c == ' ' => {}, // Skip white space
                Some(c) => match self.state {
                    Digit => {
                        self.state = Operator;
                        let tok = Token::Integer(self.stack.iter().collect::<String>().parse().unwrap());
                        self.stack.clear();
                        self.stack.push(c);
                        return Some(tok);
                    },
                    Operator => {
                        let tok = Token::Operator(self.stack.pop().unwrap());
                        self.stack.push(c);
                        return Some(tok);
                    },
                    End => return None,
                }
                None => match self.state {
                    Digit => {
                        self.state = End;
                        let tok = Token::Integer(self.stack.iter().collect::<String>().parse().unwrap());
                        self.stack.clear();
                        return Some(tok);
                    },
                    Operator => {
                        self.state = End;
                        let tok = Token::Operator(self.stack.pop().unwrap());
                        return Some(tok);
                    },
                    End => return None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("5+123");
        assert_eq!(lexer.next().unwrap(), Token::Integer(5));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115+123");
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115++123");
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115+123+1");
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(1));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("(115+123)+1");
        assert_eq!(lexer.next().unwrap(), Token::Operator('('));
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::Operator(')'));
        assert_eq!(lexer.next().unwrap(), Token::Operator('+'));
        assert_eq!(lexer.next().unwrap(), Token::Integer(1));
        assert_eq!(lexer.next(), None);
    }
}
