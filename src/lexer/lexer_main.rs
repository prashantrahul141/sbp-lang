use crate::app::app_main::App;
use crate::token::token_main::Token;
use crate::token::token_main::TokenLiterals;
use crate::token::token_types::TokenType;

pub struct Lexer {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub len: usize,
    pub tokens: Vec<Token>,
    pub source_string: String,
    pub source_chars: Vec<char>,
}

impl Lexer {
    // scans indiviual tokens.
    pub fn scan_token(&mut self) {
        let current_char = self.advance();
        match current_char {
            '(' => self.add_basic_token(TokenType::LeftParen),
            ')' => self.add_basic_token(TokenType::RightParen),
            '{' => self.add_basic_token(TokenType::LeftBrace),
            '}' => self.add_basic_token(TokenType::RightBrace),
            ',' => self.add_basic_token(TokenType::Comma),
            '.' => self.add_basic_token(TokenType::Dot),
            '-' => self.add_basic_token(TokenType::Minus),
            '+' => self.add_basic_token(TokenType::Plus),
            ';' => self.add_basic_token(TokenType::Semicolon),
            '*' => self.add_basic_token(TokenType::Star),
            '\n' => {
                self.line += 1;
                spdlog::trace!("found newline, incrementing line number and skipping.");
            }
            _ => {
                spdlog::error!(
                    "found a character which is not defined at line : {}",
                    self.line
                );
                App::error(
                    self.line,
                    "Found a character which is not defined.".to_string(),
                );
            }
        }
    }

    /// lexer scan tokens function.
    /// returns a vector of all scanned tokens.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        spdlog::debug!("scanning tokens");

        while !self.is_at_end() {
            spdlog::trace!("did not reach end, consuming.");
            self.start = self.current;
            self.scan_token();
        }

        spdlog::debug!("reached end of file, stopped consuming.");

        self.add_token(TokenType::Eof, TokenLiterals::Null);

        spdlog::debug!(
            "done scanning tokens, scanned : {} tokens.",
            self.tokens.len()
        );

        for i in &self.tokens {
            println!("{}", i);
        }

        &self.tokens
    }
}
