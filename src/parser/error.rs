use crate::token::token_main::Token;

#[derive(Debug, Clone)]
pub struct ParserError {
    // token where error was caused.
    pub token: Token,
    // error message
    pub message: String,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ParserError {
    pub fn new(token: &Token, message: String) -> Self {
        Self {
            token: token.clone(),
            message,
        }
    }
}
