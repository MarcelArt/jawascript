#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    Plus,
    Minus,
    Star,
    Slash,

    LParen,
    RParen,

    EOF,

}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn read_number(&mut self) -> Token {
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let number: String = self.input[start..self.position].iter().collect();

        Token::Number(number.parse().unwrap())
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' | '\n' => self.advance(),
                '+' => {
                    self.advance();
                    return Token::Plus;
                },
                '-' => {
                    self.advance();
                    return Token::Minus;
                },
                '*' => {
                    self.advance();
                    return Token::Star;
                },
                '/' => {
                    self.advance();
                    return Token::Slash;
                },
                '(' => {
                    self.advance();
                    return Token::LParen;
                },
                ')' => {
                    self.advance();
                    return Token::RParen;
                },
                '0'..='9' => {
                    return self.read_number();
                },
                _ => {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }

        Token::EOF
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();

            if token == Token::EOF {
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}