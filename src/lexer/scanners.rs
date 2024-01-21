use super::lexer_main::Lexer;
use crate::{
    app::app_main::App,
    token::{token_main::TokenLiterals, token_types::TokenType},
};

impl Lexer {
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

    /// Creates a token for strings.
    pub fn scan_number(&mut self) {
        spdlog::trace!("looping to parse number token.");
        // scan number until we dont get a character which is numeric
        while self.look_ahead().is_numeric() {
            // consume the numbers.
            self.advance();
        }

        // for floating point numbers.
        if self.look_ahead() == '.' && self.look_ahead_twice().is_ascii_digit() {
            spdlog::trace!(
                "found . && and numeric characters afterwards, parsing decimal part of the number."
            );

            // consume the '.'
            self.advance();

            // scan number until we dont get a character which is numeric
            while self.look_ahead().is_numeric() {
                // consume the numbers.
                self.advance();
            }
        }

        // getting the literal and parsing it into rust's f64 float.
        let literal = self.source_string[self.start..self.current]
            .to_string()
            .parse::<f64>();

        // checking for result.
        match literal {
            Ok(literal) => {
                spdlog::trace!("parsed number string to f64 number.");
                self.add_token(TokenType::Number, TokenLiterals::Number(literal));
            }
            Err(_) => {
                // printing a error and skipping the token all together.
                spdlog::error!("failed to parse number string to f64 float.");
            }
        }
    }

    /// Creates a token for identifer..
    pub fn scan_indentifier(&mut self) {
        spdlog::trace!("parsing an identifer.");
        while Lexer::is_alphanumeric(self.look_ahead()) {
            spdlog::trace!("found another alphanumber, advancing.");
            self.advance();
        }

        let lexeme = self.source_string[self.start..self.current].to_string();
        spdlog::trace!("checking for already existing keywords for : {}", lexeme);
        match self.reserved_keywords.get(&lexeme) {
            Some(reserved_keyword_type) => {
                spdlog::trace!("found match for : {}", lexeme);
                self.add_basic_token(reserved_keyword_type.clone());
            }
            None => {
                spdlog::trace!("No match found for : {}", lexeme);
                spdlog::trace!("adding token as identifer");
                self.add_basic_token(TokenType::Identifier)
            }
        }
    }
}
