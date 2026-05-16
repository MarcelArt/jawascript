#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),

    Identifier(String),

    Let,

    Equal,
    Semicolon,
    Comma,

    Plus,
    Minus,
    Star,
    Slash,

    LParen,
    RParen,

    Greater,
    Less,

    If,
    Else,
    LBrace,
    RBrace,

    While,

    Fn,

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

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn read_number(&mut self) -> Token {
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

    fn read_identifier(&mut self) -> Token {
        let start = self.position;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident: String = self.input[start..self.position].iter().collect();

        match ident.as_str() {
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "fn" => Token::Fn,
            _ => Token::Identifier(ident),
        }
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
                '=' => {
                    self.advance();
                    return Token::Equal;
                },
                ';' => {
                    self.advance();
                    return Token::Semicolon;
                },
                '>' => {
                    self.advance();
                    return Token::Greater;
                },
                '<' => {
                    self.advance();
                    return Token::Less;
                },
                '{' => {
                    self.advance();
                    return Token::LBrace;
                },
                '}' => {
                    self.advance();
                    return Token::RBrace;
                },
                ',' => {
                    self.advance();
                    return Token::Comma;
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    return self.read_identifier();
                },
                _ => {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }

        Token::EOF
    }

    #[allow(unused)]
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