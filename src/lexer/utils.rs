use std::borrow::Borrow;

use crate::token::{token_main::Token, token_main::TokenLiterals, token_types::TokenType};

use super::lexer_main::Lexer;

impl Lexer {
    // function to consume next char.
    pub fn advance(&mut self) -> char {
        let current_char = self.source_chars[self.current];
        self.current += 1;
        spdlog::trace!("advancing with current char : {}", current_char);
        current_char
    }

    // helper function to add basic tokens
    pub fn add_basic_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, TokenLiterals::Null);
    }

    // function to add a token to the tokens list.
    pub fn add_token(&mut self, token_type: TokenType, token_literal: TokenLiterals) {
        spdlog::trace!("adding token : ");
        self.tokens.push(Token::new(
            token_type,
            "".to_string(),
            token_literal,
            self.line,
        ));
    }

    // constructor for lexer.
    pub fn new(source_string: String) -> Self {
        spdlog::trace!("creating lexer.");
        Self {
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
            len: source_string.len(),
            source_string: source_string.clone(),
            source_chars: source_string.chars().collect(),
        }
    }

    // helper function to check if consumed the entire file.
    pub fn is_at_end(&self) -> bool {
        spdlog::trace!(
            "checking if reached end, current : {}, len : {}",
            self.current,
            self.len
        );
        self.current >= self.len
    }
}
