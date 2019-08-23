use std::mem::discriminant;
use super::syn::{Token, ASTNode};
use super::lexer::Lexer;

/*
 * Grammar:
 * expr: term((ADD|SUB)term)*
 * term: factor((MUL|DIV)factor)*
 * factor: (ADD|SUB)factor|INTEGER|((OPEN)expr(CLOSE))
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
                        right: Box::new(self.term()),
                    }
                },
                Token::Operator('-') => {
                    self.consume_lexer(Token::Operator('-'));
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Operator('-'),
                        right: Box::new(self.term()),
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
            // Handle unary operators
            Token::Operator('+') => {
                self.consume_lexer(Token::Operator('+'));
                ASTNode::UnaryOp{
                    op: Token::Operator('+'),
                    expr: Box::new(self.factor()),
                }
            }
            Token::Operator('-') => {
                self.consume_lexer(Token::Operator('-'));
                ASTNode::UnaryOp{
                    op: Token::Operator('-'),
                    expr: Box::new(self.factor()),
                }
            }

            // Handle parentheses
            Token::Operator('(') => {
                self.consume_lexer(Token::Operator('('));
                let res = self.expr();
                self.consume_lexer(Token::Operator(')'));
                res
            },

            // Just a number
            Token::Integer(i) => {
                self.consume_lexer(Token::Integer(0));
                ASTNode::Num(i as i64)
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
            None => {
                self.cur = Token::End;
                return Token::End
            },
        };

        return tok;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::syn::ASTNode::*;
    use crate::calculator::syn::Token::Operator;
    
    #[test]
    fn test_factor_valid() {
        let mut parser = Parser::new(Lexer::new("5"));

        assert_eq!(Num(5), parser.factor());

        parser = Parser::new(Lexer::new("-5"));
        assert_eq!(UnaryOp{op: Operator('-'), expr: Box::new(Num(5))}, parser.factor());

        parser = Parser::new(Lexer::new("+5"));
        assert_eq!(UnaryOp{op: Operator('+'), expr: Box::new(Num(5))}, parser.factor());

        parser = Parser::new(Lexer::new("+(5-2)"));
        assert_eq!(UnaryOp{op: Operator('+'), expr: Box::new(BinOp{
            left: Box::new(Num(5)),
            op: Operator('-'),
            right: Box::new(Num(2)),
        })}, parser.factor());
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_factor_invalid() {
        let mut parser = Parser::new(Lexer::new("+"));
        parser.factor();
    }

    #[test]
    fn test_term_valid() {
        let mut parser = Parser::new(Lexer::new("5*2"));
        assert_eq!(BinOp{
                left: Box::new(Num(5)),
                op: Operator('*'),
                right: Box::new(Num(2)),
        }, parser.term());

        parser = Parser::new(Lexer::new("1 * 10"));
        assert_eq!(BinOp{
                left: Box::new(Num(1)),
                op: Operator('*'),
                right: Box::new(Num(10)),
        }, parser.term());

        parser = Parser::new(Lexer::new("3*6/2"));
        assert_eq!(BinOp{
            left: Box::new(BinOp{
                left: Box::new(Num(3)),
                op: Operator('*'),
                right: Box::new(Num(6)),
            }),
            op: Operator('/'),
            right: Box::new(Num(2)),
        }, parser.term());

        parser = Parser::new(Lexer::new("5/2"));
        assert_eq!(BinOp{
                left: Box::new(Num(5)),
                op: Operator('/'),
                right: Box::new(Num(2)),
        }, parser.term());
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
        assert_eq!(BinOp{
            left: Box::new(Num(5)),
            op: Operator('+'),
            right: Box::new(Num(2)),
        }, parser.expr());

        parser = Parser::new(Lexer::new("2 * 10 -6"));
        assert_eq!(BinOp{
            left: Box::new(BinOp{
                left: Box::new(Num(2)),
                op: Operator('*'),
                right: Box::new(Num(10)),
            }),
            op: Operator('-'),
            right: Box::new(Num(6)),
        }, parser.expr());

        parser = Parser::new(Lexer::new("5+4*8"));
        assert_eq!(BinOp{
            left: Box::new(Num(5)),
            op: Operator('+'),
            right: Box::new(BinOp{
                left: Box::new(Num(4)),
                op: Operator('*'),
                right: Box::new(Num(8)),
            }),
        }, parser.expr());

        parser = Parser::new(Lexer::new("6/(2+1)"));
        assert_eq!(BinOp{
            left: Box::new(Num(6)),
            op: Operator('/'),
            right: Box::new(BinOp{
                left: Box::new(Num(2)),
                op: Operator('+'),
                right: Box::new(Num(1)),
            }),
        }, parser.expr());
    }
}
