use core::panic;

use crate::{ast::{BinaryOp, Expr, Statement}, lexer::{Lexer, Token}};

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

    #[allow(unused)]
    pub fn parse(&mut self) -> Expr {
        self.parse_expression() 
    }

    pub fn parse_statement(&mut self) -> Statement {
        match self.current_token.clone() {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            _ => Statement::Expr(self.parse_comparison()),
        }
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }

        statements
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
            Token::Identifier(name) => {
                self.advance();
                Expr::Variable(name)
            }
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

    fn parse_let_statement(&mut self) -> Statement {
        self.advance();

        let name = match self.current_token.clone() {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier"),
        };

        self.advance();

        match self.current_token {
            Token::Equal => self.advance(),
            _ => panic!("Expected '='"),
        }

        let value = self.parse_expression();

        match self.current_token {
            Token::Semicolon => self.advance(),
            _ => panic!("Expected ';'",),
        }

        Statement::Let { name, value }
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_expression();

        loop {
            let op = match self.current_token {
                Token::Greater => BinaryOp::Greater,
                Token::Less => BinaryOp::Less,
                _ => break,
            };

            self.advance();

            let right = self.parse_expression();

            expr = Expr::Binary { 
                left: Box::new(expr), 
                op, 
                right: Box::new(right),
            };
        }

        expr
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.advance();

        let condition = self.parse_comparison();

        match self.current_token {
            Token::LBrace => self.advance(),
            _ => panic!("Expected '{{'"),
        }

        let mut then_branch = Vec::new();

        let else_branch = if self.current_token == Token::Else {
            self.advance();

            match self.current_token {
                Token::LBrace => self.advance(),
                _ => panic!("Expected '{{'"),
            }

            let mut statements = Vec::new();

            while self.current_token != Token::RBrace {
                statements.push(self.parse_statement());
            }

            self.advance();

            Some(statements)
        } else {
            None
        };

        while self.current_token != Token::RBrace {
            then_branch.push(self.parse_statement());
        }

        self.advance();

        Statement::If { condition, then_branch, else_branch }
    }
}