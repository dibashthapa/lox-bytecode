use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        let source = source.chars().collect();
        Self {
            start: 0,
            current: 0,
            line: 1,
            source,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Plus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                if self.is_matched('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_matched('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_matched('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_matched('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '=' => self.string(),
            '0'..='9' => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => self.error_token("Unexpected character."),
        };

        return self.error_token("Unexpected Character.");
    }

    fn identifier(&mut self) -> Token {
        while self.peek_is_alphanumeric() {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn peek_is_alphanumeric(&self) -> bool {
        let c = self.peek();
        c.is_alphanumeric() || c == '_'
    }

    fn check_keyword(
        &self,
        start: usize,
        _length: usize,
        rest: &str,
        ttype: TokenType,
    ) -> TokenType {
        let compare: String = self.source[self.start + start..self.current]
            .iter()
            .collect();

        if compare.as_str() == rest {
            return ttype;
        }

        TokenType::Identifier
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, 3, "is", TokenType::This),
                        'r' => self.check_keyword(2, 1, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }
    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            let ch = self.peek_next();
            if ch.is_digit(10) {
                self.advance();
                while self.peek().is_digit(10) {
                    self.advance();
                }
            }
        }

        self.make_token(TokenType::Number)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source[self.current + 1];
    }

    fn peek(&self) -> char {
        self.source[self.current]
    }

    fn is_matched(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&mut self, ttype: TokenType) -> Token {
        Token {
            ttype,
            lexeme: self.source[self.start..self.current].iter().collect(),
            line: self.line,
        }
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            ttype: TokenType::TokenError,
            lexeme: message.to_string(),
            line: self.line,
        }
    }
}
