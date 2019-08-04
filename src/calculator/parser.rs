use std::mem::discriminant;
use super::syn::{Token, ASTNode};
use super::lexer::Lexer;

/*
 * Grammar:
 * expr: term((ADD|SUB)term)*
 * term: factor((MUL|DIV)factor)*
 * factor: INTEGER|((OPEN)expr(CLOSE))
 */

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur: Token,
}

impl Parser<'_> {
    // Initializes a new interpreter with the start state
    pub fn new(mut lexer: Lexer) -> Parser {
        let tok = lexer.next().unwrap();
        Parser {
            lexer: lexer,
            cur: tok,
        }
    }

    pub fn expr(&mut self) -> ASTNode {
        let mut left = self.term();

        loop {
            match self.cur {
                Token::Operator('+') => {
                    self.consume_lexer(Token::Operator('+'));
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Operator('+'),
                        right: Box::new(self.factor()),
                    }
                },
                Token::Operator('-') => {
                    self.consume_lexer(Token::Operator('-'));
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Operator('-'),
                        right: Box::new(self.factor()),
                    }
                },
                _ => return left,
            }
        }
    }

    fn term(&mut self) -> ASTNode {
        let mut left = self.factor();

        loop {
            match self.cur {
                Token::Operator('*') => {
                    self.consume_lexer(Token::Operator('*'));
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Operator('*'),
                        right: Box::new(self.factor()),
                    }
                },
                Token::Operator('/') => {
                    self.consume_lexer(Token::Operator('/'));
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Operator('/'),
                        right: Box::new(self.factor()),
                    }
                },
                _ => return left,
            }
        }
    }

    fn factor(&mut self) -> ASTNode {
        match self.cur {
            Token::Operator('(') => {
                self.consume_lexer(Token::Operator('('));
                let res = self.expr();
                self.consume_lexer(Token::Operator(')'));
                res
            },
            Token::Integer(i) => {
                self.consume_lexer(Token::Integer(0));
                return ASTNode::Num{
                    value: i
                };
            },
            _ => panic!("Syntax error"),
        }
    }

    fn consume_lexer(&mut self, expect: Token) -> Token {
        if discriminant(&self.cur) != discriminant(&expect) {
            panic!("Unexpected token");
        }

        let tok = self.cur;

        match self.lexer.next() {
            Some(tok) => self.cur = tok,
            None => return Token::End,
        };

        return tok;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    macro_rules! assert_enum_eq {
        ($a:expr,$b:expr) => {
            assert_eq!(discriminant($a), discriminant($b));
        };
    }

    #[test]
    fn test_factor_valid() {
        let mut parser = Parser::new(Lexer::new("5"));

        assert_eq!(5, parser.factor());
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_factor_invalid() {
        let mut parser = Parser::new(Lexer::new("+"));
        parser.factor();
        parser.factor();
    }

    #[test]
    fn test_term_valid() {
        let mut parser = Parser::new(Lexer::new("5*2"));
        assert_eq!(10, parser.term());

        parser = Parser::new(Lexer::new("1 * 10"));
        assert_eq!(10, parser.term());

        parser = Parser::new(Lexer::new("3*6/2"));
        assert_eq!(9, parser.term());

        parser = Parser::new(Lexer::new("5/2"));
        assert_eq!(2, parser.term());
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_term_invalid() {
        let mut parser = Parser::new(Lexer::new("5**2"));
        parser.term();
    }

    #[test]
    fn test_expr_valid() {
        let mut parser = Parser::new(Lexer::new("5+2"));
        assert_eq!(7, parser.expr());

        parser = Parser::new(Lexer::new("2 * 10 -6"));
        assert_eq!(14, parser.expr());

        parser = Parser::new(Lexer::new("5+4*8"));
        assert_eq!(37, parser.expr());

        parser = Parser::new(Lexer::new("6/2+1"));
        assert_eq!(4, parser.expr());

        parser = Parser::new(Lexer::new("7 + 3 * (10 / (12 / (3 + 1) - 1))"));
        assert_eq!(22, parser.expr());
    }

}
