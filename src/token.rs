use std::fmt;

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    TokenError,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    SingleLineColumn,
    Slash,
    Star,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize
}


