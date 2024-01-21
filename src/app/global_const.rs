use std::collections::HashMap;

use crate::token::token_types::TokenType;

use super::app_main::App;

impl App {
    /// Gets the reserved keywords.
    pub fn get_reserved_keywords() -> HashMap<String, TokenType> {
        spdlog::debug!("generating and returning reserved keywords hashmap.");
        HashMap::from([
            ("true".to_string(), TokenType::True),
            ("false".to_string(), TokenType::False),
            ("null".to_string(), TokenType::Null),
            ("and".to_string(), TokenType::And),
            ("or".to_string(), TokenType::Or),
            ("let".to_string(), TokenType::Let),
            ("class".to_string(), TokenType::Class),
            ("fn".to_string(), TokenType::Fn),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("if".to_string(), TokenType::If),
            ("else".to_string(), TokenType::Else),
            ("while".to_string(), TokenType::While),
            ("for".to_string(), TokenType::For),
            ("print".to_string(), TokenType::Print),
        ])
    }
}
