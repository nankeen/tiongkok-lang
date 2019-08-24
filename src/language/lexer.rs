use super::syn::Token;
use std::iter::Peekable;


pub struct Lexer<I: Iterator<Item=char>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item=char>> Lexer<I> {
    pub fn new(s: I) -> Lexer<I> {
        Lexer {
            iter: s.peekable(),
        }
    }

    fn lex_integer(&mut self, first: char) -> Token {
        let mut stack: String = first.to_string();
        while let Some(d) = self.iter.peek() {
            if d.is_numeric() {
                stack.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        Token::Integer(stack.parse().unwrap())
    }

    fn lex_identifier(&mut self, first: char) -> Token {
        let mut stack: String = first.to_string();
        while let Some(d) = self.iter.peek() {
            if d.is_alphabetic() {
                stack.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        Token::Id(stack)
    }
}

impl<I: Iterator<Item=char>> Iterator for Lexer<I> {
    type Item=Token;
    fn next(&mut self) -> Option<Token> {
        match self.iter.next() {
            Some(' ') => self.next(),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some('{') => Some(Token::LCBrac),
            Some('}') => Some(Token::RCBrac),
            Some(':') => {
                match self.iter.peek() {
                    Some('=') => {
                        self.iter.next();
                        Some(Token::Assign)
                    },
                    _ => None
                }
            }
            Some(c) if c.is_numeric() => Some(self.lex_integer(c)),
            Some(c) if c.is_alphabetic() => Some(self.lex_identifier(c)),
            _ => return None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_expr() {
        let mut lexer = Lexer::new("5+123".chars());
        assert_eq!(lexer.next().unwrap(), Token::Integer(5));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115+123".chars());
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115++123".chars());
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("115+123+1".chars());
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(1));
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("(115+123)+1".chars());
        assert_eq!(lexer.next().unwrap(), Token::LParen);
        assert_eq!(lexer.next().unwrap(), Token::Integer(115));
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::RParen);
        assert_eq!(lexer.next().unwrap(), Token::Add);
        assert_eq!(lexer.next().unwrap(), Token::Integer(1));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_lexer_prog() {
        let mut lexer = Lexer::new("{ asd := 123}".chars());
        assert_eq!(lexer.next().unwrap(), Token::LCBrac);
        assert_eq!(lexer.next().unwrap(), Token::Id("asd".to_owned()));
        assert_eq!(lexer.next().unwrap(), Token::Assign);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::RCBrac);
        assert_eq!(lexer.next(), None);

        lexer = Lexer::new("{ asd:= 123}".chars());
        assert_eq!(lexer.next().unwrap(), Token::LCBrac);
        assert_eq!(lexer.next().unwrap(), Token::Id("asd".to_owned()));
        assert_eq!(lexer.next().unwrap(), Token::Assign);
        assert_eq!(lexer.next().unwrap(), Token::Integer(123));
        assert_eq!(lexer.next().unwrap(), Token::RCBrac);
        assert_eq!(lexer.next(), None);
    }
}
