use super::token_types::TokenType;

/// The Token struct, holds info about a single token.
#[derive(Debug, Clone)]
pub struct Token {
    // type of token.
    pub token_type: TokenType,
    // its string lexem.
    pub lexeme: String,
    // the literal value of the token.
    pub literal: TokenLiterals,
    // line number in the source file.
    pub line: usize,
}

/// enum for token literals.
#[derive(Debug, Clone)]
pub enum TokenLiterals {
    // if the token literal is integer or float.
    Number(f64),
    // if the token literal is string.
    String(String),
    // if the token literal is boolean.
    Boolean(bool),
    // all other token types.
    Null,
}

impl Token {
    /// creates and returns new instance of token struct.
    /// # Arguments
    /// * `token_type` - Type of the token to create.
    /// * `lexem` - token's lexeme value.
    /// * `literal` - literal value of the token.
    /// * `line` - line number in the file token was present.
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

/// Using Debug's implementation as the Display implementation for Token.
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// display implementation for token literals.
impl std::fmt::Display for TokenLiterals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenLiterals::Number(n) => write!(f, "{}", n),
            TokenLiterals::String(s) => write!(f, "{}", s),
            TokenLiterals::Boolean(s) => write!(f, "{}", s),
            TokenLiterals::Null => write!(f, "null"),
        }
    }
}
