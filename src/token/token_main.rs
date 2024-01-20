use super::token_types::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiterals,
    pub line: usize,
}

#[derive(Debug)]
pub enum TokenLiterals {
    Number(f64),
    String(String),
    Null,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: TokenLiterals, line: usize) -> Self {
        spdlog::trace!(
            "creating token with type: {}, lexeme : {}, literal : {}, line : {}",
            token_type,
            lexeme,
            literal,
            line
        );

        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// using debug macro to generate display fmt and then use that as its actual implementation.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// using debug macro to generate display fmt and then use that as its actual implementation.
impl std::fmt::Display for TokenLiterals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
