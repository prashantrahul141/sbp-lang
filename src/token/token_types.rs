#[derive(Debug, Clone)]
pub enum TokenType {
    // single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords
    True,
    False,
    Null,
    And,
    Or,
    Let,
    Class,
    Fn,
    Return,
    Super,
    This,
    If,
    Else,
    While,
    For,
    Print,

    // end of file.
    Eof,
}

// using debug macro to generate display fmt and then use that as its actual implementation.
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
