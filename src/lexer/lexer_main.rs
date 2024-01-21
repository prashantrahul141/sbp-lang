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
            // single character tokens.
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

            // multiple character basic tokens.
            // !
            '!' => {
                if self.match_char('=') {
                    // !=
                    self.add_basic_token(TokenType::BangEqual);
                } else {
                    // =
                    self.add_basic_token(TokenType::Bang);
                }
            }

            // =
            '=' => {
                if self.match_char('=') {
                    // ==
                    self.add_basic_token(TokenType::EqualEqual);
                } else {
                    // =
                    self.add_basic_token(TokenType::Equal);
                }
            }

            // <
            '<' => {
                if self.match_char('=') {
                    // <=
                    self.add_basic_token(TokenType::LessEqual);
                } else {
                    // <
                    self.add_basic_token(TokenType::Less);
                }
            }

            // >
            '>' => {
                if self.match_char('=') {
                    // >=
                    self.add_basic_token(TokenType::GreaterEqual);
                } else {
                    // >
                    self.add_basic_token(TokenType::Greater);
                }
            }

            // longer lexemes
            // comments
            '/' => {
                if self.match_char('/') {
                    // keep consuming characters untill we reach the end of file or newline.
                    while self.look_ahead() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    // we dont add any token for comments BECAUSE they're comments.
                } else {
                    // its a simple slash.
                    self.add_basic_token(TokenType::Slash);
                }
            }

            ' ' | '\t' | '\r' => {
                // ignore these characters, we dont need'em.
                spdlog::trace!("ignoring whitespaces");
            }

            // newline, its basically a single line character.
            '\n' => {
                self.line += 1;
                spdlog::trace!("found newline, incrementing line number and skipping.");
            }

            // reporting error but keep scanning if found an unexpected character.
            _ => {
                spdlog::error!(
                    "unexpected character : {} at line : {}",
                    current_char,
                    self.line
                );
                App::error(
                    self.line,
                    format!("unexpected character : {}", current_char),
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
