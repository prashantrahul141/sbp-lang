use super::lexer_main::Lexer;
use crate::{
    app::app_main::App,
    token::{token_main::Token, token_main::TokenLiterals, token_types::TokenType},
};

impl Lexer {
    /// creates and returns a new instance of lexer struct.
    /// # Arguments
    /// * `source_string` - input source string.
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

    /// Consumes one character and returns it.
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

    /// Add a token to the global tokens vector of token_type given and Null as token literal value.
    /// # Arguments
    /// * `token_type` - Type of the token to add.
    pub fn add_basic_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, TokenLiterals::Null);
    }

    /// Adds a token to the tokens list.
    /// # Arguments
    /// * `token_type` - Type of the token to add.
    /// * `token_literal` - Token literal
    pub fn add_token(&mut self, token_type: TokenType, token_literal: TokenLiterals) {
        spdlog::trace!("adding token : ");
        let lexeme = self.source_string[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, token_literal, self.line));
    }

    /// Check and returns if reached the end of the input source string.
    pub fn is_at_end(&self) -> bool {
        spdlog::trace!(
            "checking if reached end, current : {}, len : {}",
            self.current,
            self.len
        );
        self.current >= self.len
    }

    /// Returns the next char in input source string but doesnt consume it.
    /// returns '\0' if reached the end of file.
    pub fn look_ahead(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source_chars[self.current]
    }

    /// Creates a token for strings.
    pub fn scan_string(&mut self) {
        // looping until another " is found or end of file is found.
        spdlog::trace!("looping to parse string token.");
        while self.look_ahead() != '"' && !self.is_at_end() {
            // incrementing line number whenever newline is found.
            if self.look_ahead() == '\n' {
                spdlog::trace!("found newline inside a string.");
                self.line += 1;
            }

            // consuming character.
            self.advance();
        }

        // if reached the end without a "
        if self.is_at_end() {
            spdlog::error!("found a unterminated string, returning.");
            App::error(self.line, "Unterminated strings.".to_string());
            return;
        }

        // consume the ending "
        spdlog::trace!("consuming ending \"");
        self.advance();

        // trim the surrounding quotes.
        let literal = self.source_string[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, TokenLiterals::String(literal));
    }
}
