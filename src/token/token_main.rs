use super::token_types::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Self {
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
