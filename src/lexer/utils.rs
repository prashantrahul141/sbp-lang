use super::lexer_main::Lexer;
use crate::token::{token_main::Token, token_main::TokenLiterals, token_types::TokenType};

impl Lexer {
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

    // function to consume next char and return it.
    pub fn advance(&mut self) -> char {
        let current_char = self.source_chars[self.current];
        self.current += 1;
        spdlog::trace!("advancing with current char : {}", current_char);
        current_char
    }

    /// Return true and consumes the next character if it matches the given
    /// character, otherwise returns false.
    ///
    /// # Arguments
    /// * `expected` - a character which needs to be matched against next character.
    pub fn match_char(&mut self, expected: char) -> bool {
        // check if reached the end.
        if self.is_at_end() {
            return false;
        }

        // false if the next char is not expected.
        if self.source_chars[self.current] != expected {
            return false;
        }

        // only consume the character if it does matches the expected character.
        self.current += 1;
        true
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

    // helper function to check if consumed the entire file.
    pub fn is_at_end(&self) -> bool {
        spdlog::trace!(
            "checking if reached end, current : {}, len : {}",
            self.current,
            self.len
        );
        self.current >= self.len
    }

    /// Helper function to lookahead a character without consuming it.
    // returns \0 if reached the end of file.
    pub fn look_ahead(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source_chars[self.current]
    }
}
