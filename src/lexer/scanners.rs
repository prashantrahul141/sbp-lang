use super::lexer_main::Lexer;
use crate::{
    app::app_main::App,
    token::{token_main::TokenLiterals, token_types::TokenType},
};

impl Lexer {
    /// Creates a token for strings.
    pub fn scan_string(&mut self) {
        spdlog::trace!("looping to parse string token.");

        // looping until another " is found or reaced end of file.
        while self.look_ahead() != '"' && !self.is_at_end() {
            // incrementing line number whenever newline is found.
            if self.look_ahead() == '\n' {
                spdlog::trace!("found newline inside a string, incrementing line count.");
                self.line += 1;
            }

            // consuming character.
            self.advance();
        }

        // if reached the end without a "
        if self.is_at_end() {
            App::error(self.line, "Unterminated string.".to_string());
            return;
        }

        // consume the ending "
        spdlog::trace!("consuming ending \"");
        self.advance();

        // trim the surrounding quotes.
        let literal = self.source_string[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, TokenLiterals::String(literal));
    }

    /// Creates a token for numbers
    pub fn scan_number(&mut self) {
        spdlog::trace!("looping to parse number token.");
        // scan number until we dont get a character which is numeric
        while Lexer::is_numeric(self.look_ahead()) {
            // consume the numeric character.
            self.advance();
        }

        // for floating point numbers.
        if self.look_ahead() == '.' && Lexer::is_numeric(self.look_ahead_twice()) {
            spdlog::trace!(
                "found . && and numeric characters afterwards, parsing decimal part of the number."
            );

            // consume the '.'
            self.advance();

            // scan number until we dont get a character which is numeric
            while Lexer::is_numeric(self.look_ahead()) {
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
                App::error(self.line, "failed to parse into number.".to_string());
            }
        }
    }

    /// Creates a token for identifer..
    pub fn scan_indentifier(&mut self) {
        spdlog::trace!("parsing an identifer.");
        while Lexer::is_alphanumeric(self.look_ahead()) {
            spdlog::trace!("found another alphanumeric char, advancing.");
            self.advance();
        }

        // lexeme string of the identifier.
        let lexeme = self.source_string[self.start..self.current].to_string();
        spdlog::trace!("checking for already existing keywords for : {}", lexeme);

        // check if the identifier is a reserved keyword.
        match self.reserved_keywords.get(&lexeme) {
            Some(reserved_keyword_type) => {
                spdlog::trace!("found match for : {}", lexeme);
                self.add_basic_token(reserved_keyword_type.clone());
            }
            None => {
                spdlog::trace!(
                    "No match found for : {}, adding token as identifer.",
                    lexeme
                );
                self.add_basic_token(TokenType::Identifier)
            }
        }
    }
}
