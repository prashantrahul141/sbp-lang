use super::{error::ParserError, parser_main::Parser};
use crate::{
    app::app_main::App,
    token::{token_main::Token, token_types::TokenType},
};

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        spdlog::debug!("creating new parser");
        Self { tokens, current: 0 }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) -> Option<&Token> {
        spdlog::trace!("consume called for TokenType : {token_type} and message : {message}");
        if self.check(&token_type) {
            spdlog::trace!("check failed, returning");
            return Some(self.advance());
        }

        self.error(self.peek(), message);
        None
    }

    pub fn error(&self, token: &Token, message: String) -> ParserError {
        App::error_token(token.clone(), message);
        ParserError::new()
    }

    /// Takes in a vec of TokenTypes, compares it with current type of the Token
    /// calls self.advance() on it and returns true if any matches.
    /// else returns false.
    /// # Arguments
    /// * `tokens` - Vector of tokens.
    pub fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        spdlog::trace!(
            "matching current token : {} with tokens : {:?}",
            self.tokens[self.current],
            token_types
        );
        for token in token_types.iter() {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consumes the current token and returns it, if not reached the end.
    pub fn advance(&mut self) -> &Token {
        spdlog::trace!(
            "advancing and consuming current token : {}",
            self.tokens[self.current]
        );
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Returns the previous token.
    pub fn previous(&mut self) -> &Token {
        spdlog::trace!("previous token : {}", self.tokens[self.current - 1]);
        &self.tokens[self.current - 1]
    }

    /// Checks and returns if current token is of given argument's type.
    /// # Arguments
    /// * `token` - The TokenType to compare.
    pub fn check(&self, token_type: &TokenType) -> bool {
        spdlog::trace!(
            "checking if current token : {} is of type : {}",
            self.peek(),
            token_type
        );
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type.to_owned()
    }

    /// Returns the next token without consuming it.
    pub fn peek(&self) -> &Token {
        let peeking_token = &self.tokens[self.current];
        peeking_token
    }

    /// Check and returns if the next token is of EOF.
    pub fn is_at_end(&self) -> bool {
        let reached_end = self.peek().token_type == TokenType::Eof;
        spdlog::trace!("checking if reached the end : {}", reached_end);
        reached_end
    }

    pub fn synchronize(&mut self) {
        spdlog::trace!("trying to synchronize.");
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                spdlog::trace!("found Semicolon, synchronizing.");
                return;
            }

            match self.peek().token_type {
                TokenType::Let
                | TokenType::Class
                | TokenType::Fn
                | TokenType::Return
                | TokenType::If
                | TokenType::While
                | TokenType::For
                | TokenType::Print => {
                    spdlog::trace!("found statement starting indentifier, synchronizing.");
                    return;
                }
                _ => (),
            }

            self.advance();
        }
    }
}
