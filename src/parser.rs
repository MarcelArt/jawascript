use crate::{ast::{BinaryOp, Expr}, lexer::{Lexer, Token}};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();

        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current_token.clone() {
            Token::Number(value) => {
                self.advance();
                Expr::Number(value)
            },
            Token::LParen => {
                self.advance();

                let expr = self.parse_expression();

                match self.current_token {
                    Token::RParen => {
                        self.advance();
                        expr
                    },
                    _ => panic!("Expected ')'"),
                }
            },
            _ => panic!("Unexpected token {:?}", self.current_token),
        }
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        loop {
            match self.current_token {
                Token::Star => {
                    self.advance();

                    let right = self.parse_factor();

                    left = Expr::Binary { 
                        left: Box::new(left), 
                        op: BinaryOp::Mul, 
                        right: Box::new(right) 
                    };
                },
                Token::Slash => {
                    self.advance();

                    let right = self.parse_factor();

                    left = Expr::Binary { 
                        left: Box::new(left), 
                        op: BinaryOp::Div, 
                        right: Box::new(right) 
                    };
                },
                _ => break,
            }
        }

        left
    }

    fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        loop {
            match self.current_token {
                Token::Plus => {
                    self.advance();

                    let right = self.parse_term();

                    left = Expr::Binary { 
                        left: Box::new(left), 
                        op: BinaryOp::Add, 
                        right: Box::new(right) 
                    };
                },
                Token::Minus => {
                    self.advance();

                    let right = self.parse_term();

                    left = Expr::Binary { 
                        left: Box::new(left), 
                        op: BinaryOp::Sub, 
                        right: Box::new(right) 
                    };
                },
                _ => break,
            }
        }
        left
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
        
    }
}