use super::syn::Token;

/*
 * integer: DIGIT+
 * operator: !DIGIT|MULTIPLY|DIVIDE|MINUS|PLUS
 */


enum LexerState {
    Digit,
    Operator,
    End,
}

pub struct Lexer<'a> {
    cur: Option<char>,
    iter: std::str::Chars<'a>,
}

impl Lexer<'_> {
    pub fn new(s: &str) -> Lexer {
        // Stack contains '0' to make starting with operators a valid expression
        let mut c = s.chars();
        Lexer {
            cur: c.next(), 
            iter: c,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut stack = String::new();
        let mut alpha_stack = String::new();
        loop {
            match self.cur {
                Some(' ') => {
                    // Ignore whitespace
                    self.cur = self.iter.next();
                },
                Some(c) if c.is_digit(10) => {
                    // Digit
                    stack.push(c);
                    self.cur = self.iter.next();
                },
                Some(c) if stack.is_empty() => {
                    // Operator
                    if c.is_alphabetic() {
                        alpha_stack.push(c);
                        self.cur = self.iter.next();

                        match alpha_stack.to_ascii_lowercase().as_ref() {
                            "plus" => return Some(Token::Operator('+')),
                            "minue" => return Some(Token::Operator('-')),
                            "times" => return Some(Token::Operator('*')),
                            "divide" => return Some(Token::Operator('/')),
                            _ => {},
                        };
                    } else {
                        self.cur = self.iter.next();
                        return Some(Token::Operator(c));
                    }
                },
                None if stack.is_empty() => return None,
                _ => {
                    // Return integer
                    let tok = Token::Integer(stack.parse().unwrap());
                    return Some(tok);
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
