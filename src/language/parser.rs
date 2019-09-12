use std::mem::discriminant;
use super::syn::{Token, ASTNode};
use super::lexer::Lexer;

/*
 * Grammar:
 * program: compound_statement
 * compound_statement: LCBRAC statement_list RCBRAC
 * statement_list: statement
 *               | statement SEMI statement_list
 * statement: compound_statement
 *          | assign_statement
 *          | empty
 * assign_statement: variable ASSIGN expr
 * empty: 
 *
 * expr: term((ADD|SUB)term)*
 * term: factor((MUL|DIV)factor)*
 * factor: (ADD|SUB)factor
 *       | INTEGER
 *       | LPAREN expr RPAREN
 *       | variable
 * variable: IDENTIFIER
 */

pub struct Parser <I: Iterator<Item=char>>{
    lexer: Lexer<I>,
    cur: Token,
}

impl<I: Iterator<Item=char>> Parser<I>{
    // Initializes a new interpreter with the start state
    pub fn new(mut lexer: Lexer<I>) -> Parser<I> {
        let tok = lexer.next().unwrap();
        Parser {
            lexer: lexer,
            cur: tok,
        }
    }

    pub fn parse(&mut self) -> ASTNode {
        let node = self.program();
        if let Token::EOF = self.cur {
            return node;
        }
        panic!("Expected EOF")
    }

    fn program(&mut self) -> ASTNode {
        self.compound_statement()
    }

    /*
     * Compound statement grammar
     * compound_statement: LCBRAC statement_list RCBRAC
     */
    fn compound_statement(&mut self) -> ASTNode {

        // Get children from statement list
        self.consume_lexer(Token::LCBrac);
        let children = self.statement_list();
        self.consume_lexer(Token::RCBrac);

        // Return Compound node
        ASTNode::Compound{
            children: children
        }
    }

    /*
     * Statement list grammar
     * statement_list: statement
     *               | statement SEMI statement_list
     */
    fn statement_list(&mut self) -> Vec<ASTNode> {
        let mut results = vec![self.statement()];
        while let Token::Semi = self.cur {
            self.consume_lexer(Token::Semi);
            results.push(self.statement());
        }
        return results;
    }

    /*
     * Statement grammar
     * statement: compound_statement
     *          | assign_statement
     *          | empty
     */
    fn statement(&mut self) -> ASTNode {
        match self.cur {
            Token::LCBrac => self.compound_statement(),
            Token::Id(_) => self.assign_statement(),
            _ => self.empty(),
        }
    }

    /*
     * Assign grammar
     * assign_statement: variable ASSIGN expr
     */
    fn assign_statement(&mut self) -> ASTNode {
        let left = self.variable();
        self.consume_lexer(Token::Assign);
        ASTNode::Assign {
            left: Box::new(left),
            right: Box::new(self.expr()),
        }
    }

    /*
     * variable: IDENTIFIER
     */
    fn variable(&mut self) -> ASTNode {
        ASTNode::Var {
            token: self.consume_lexer(Token::Id("Id".to_owned())),
        }
    }

    fn empty(&mut self) -> ASTNode {
        ASTNode::NoOp
    }

    /*
     * Expression grammar
     * expr: term((ADD|SUB)term)*
     */
    fn expr(&mut self) -> ASTNode {
        let mut left = self.term();

        loop {
            match self.cur {
                Token::Add => {
                    self.consume_lexer(Token::Add);
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Add,
                        right: Box::new(self.term()),
                    }
                },
                Token::Sub => {
                    self.consume_lexer(Token::Sub);
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Sub,
                        right: Box::new(self.term()),
                    }
                },
                _ => return left,
            }
        }
    }

    /*
     * Term grammar
     * term: factor((MUL|DIV)factor)*
     */
    fn term(&mut self) -> ASTNode {
        let mut left = self.factor();

        loop {
            match self.cur {
                Token::Mul => {
                    self.consume_lexer(Token::Mul);
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Mul,
                        right: Box::new(self.factor()),
                    }
                },
                Token::Div => {
                    self.consume_lexer(Token::Div);
                    left = ASTNode::BinOp{
                        left: Box::new(left),
                        op: Token::Div,
                        right: Box::new(self.factor()),
                    }
                },
                _ => return left,
            }
        }
    }

    /*
     * Factor grammar
     * factor: (ADD|SUB)factor
     *       | INTEGER
     *       | LPAREN expr RPAREN
     *       | variable
     */
    fn factor(&mut self) -> ASTNode {
        match self.cur.clone() {
            // Handle unary operators
            Token::Add => {
                self.consume_lexer(Token::Add);
                ASTNode::UnaryOp{
                    op: Token::Add,
                    expr: Box::new(self.factor()),
                }
            }
            Token::Sub => {
                self.consume_lexer(Token::Sub);
                ASTNode::UnaryOp{
                    op: Token::Sub,
                    expr: Box::new(self.factor()),
                }
            }

            // Handle parentheses
            Token::LParen => {
                self.consume_lexer(Token::LParen);
                let res = self.expr();
                self.consume_lexer(Token::RParen);
                res
            },

            // Just a number
            Token::Integer(i) => {
                self.consume_lexer(Token::Integer(0));
                ASTNode::Num(i as i64)
            },
            Token::Id(id) => {
                ASTNode::Var {
                    token: Token::Id(id),
                }
            }
            _ => panic!("Syntax error"),
        }
    }

    // Helper function to check for the correct token
    fn consume_lexer(&mut self, expect: Token) -> Token {
        if discriminant(&self.cur) != discriminant(&expect) {
            panic!("Unexpected token");
        }

        let tok = self.cur.clone();

        match self.lexer.next() {
            Some(tok) => self.cur = tok,
            None => {
                self.cur = Token::EOF;
                return Token::EOF;
            },
        };

        return tok;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::syn::ASTNode::*;
    use crate::language::syn::Token::*;
    
    #[test]
    fn test_factor_valid() {
        let mut parser = Parser::new(Lexer::new("5".chars()));

        assert_eq!(Num(5), parser.factor());

        parser = Parser::new(Lexer::new("-5".chars()));
        assert_eq!(UnaryOp{op: Sub, expr: Box::new(Num(5))}, parser.factor());

        parser = Parser::new(Lexer::new("+5".chars()));
        assert_eq!(UnaryOp{op: Add, expr: Box::new(Num(5))}, parser.factor());

        parser = Parser::new(Lexer::new("+(5-2)".chars()));
        assert_eq!(UnaryOp{op: Add, expr: Box::new(BinOp{
            left: Box::new(Num(5)),
            op: Sub,
            right: Box::new(Num(2)),
        })}, parser.factor());
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_factor_invalid() {
        let mut parser = Parser::new(Lexer::new("+".chars()));
        parser.factor();
    }

    #[test]
    fn test_term_valid() {
        let mut parser = Parser::new(Lexer::new("5*2".chars()));
        assert_eq!(BinOp{
                left: Box::new(Num(5)),
                op: Mul,
                right: Box::new(Num(2)),
        }, parser.term());

        parser = Parser::new(Lexer::new("1 * 10".chars()));
        assert_eq!(BinOp{
                left: Box::new(Num(1)),
                op: Mul,
                right: Box::new(Num(10)),
        }, parser.term());

        parser = Parser::new(Lexer::new("3*6/2".chars()));
        assert_eq!(BinOp{
            left: Box::new(BinOp{
                left: Box::new(Num(3)),
                op: Mul,
                right: Box::new(Num(6)),
            }),
            op: Div,
            right: Box::new(Num(2)),
        }, parser.term());

        parser = Parser::new(Lexer::new("5/2".chars()));
        assert_eq!(BinOp{
                left: Box::new(Num(5)),
                op: Div,
                right: Box::new(Num(2)),
        }, parser.term());
    }

    #[test]
    #[should_panic(expected = "Syntax error")]
    fn test_term_invalid() {
        let mut parser = Parser::new(Lexer::new("5**2".chars()));
        parser.term();
    }

    #[test]
    fn test_expr_valid() {
        let mut parser = Parser::new(Lexer::new("5+2".chars()));
        assert_eq!(BinOp{
            left: Box::new(Num(5)),
            op: Add,
            right: Box::new(Num(2)),
        }, parser.expr());

        parser = Parser::new(Lexer::new("2 * 10 -6".chars()));
        assert_eq!(BinOp{
            left: Box::new(BinOp{
                left: Box::new(Num(2)),
                op: Mul,
                right: Box::new(Num(10)),
            }),
            op: Sub,
            right: Box::new(Num(6)),
        }, parser.expr());

        parser = Parser::new(Lexer::new("5+4*8".chars()));
        assert_eq!(BinOp{
            left: Box::new(Num(5)),
            op: Add,
            right: Box::new(BinOp{
                left: Box::new(Num(4)),
                op: Mul,
                right: Box::new(Num(8)),
            }),
        }, parser.expr());

        parser = Parser::new(Lexer::new("6/(2+1)".chars()));
        assert_eq!(BinOp{
            left: Box::new(Num(6)),
            op: Div,
            right: Box::new(BinOp{
                left: Box::new(Num(2)),
                op: Add,
                right: Box::new(Num(1)),
            }),
        }, parser.expr());
    }

    #[test]
    fn test_unary_operator() {
        let mut parser = Parser::new(Lexer::new("-3".chars()));
        assert_eq!(UnaryOp{
            op: Sub,
            expr: Box::new(Num(3)),
        }, parser.expr());

        parser = Parser::new(Lexer::new("+3".chars()));
        assert_eq!(UnaryOp{
            op: Add,
            expr: Box::new(Num(3)),
        }, parser.expr());
    }
}
