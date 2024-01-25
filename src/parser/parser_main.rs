use std::vec;

use crate::{
    app::app_main::App,
    ast::{
        expr_ast::{Expr, ExprBinary, ExprGrouping, ExprLiteral, ExprUnary, ExprVariable},
        stmt_ast::{Stmt, StmtExpr, StmtLet, StmtPrint},
    },
    token::{
        self,
        token_main::{Token, TokenLiterals},
        token_types::TokenType,
    },
};

/// Top level parser struct.
pub struct Parser {
    // vector of tokens to parse.
    pub tokens: Vec<Token>,
    // current progress of parsing.
    pub current: usize,
}

impl Parser {
    /// Top level method which handles all parsing and returns vec
    /// of statements. Skips the statements which found to be faulty,
    /// while giving user errors.
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];
        spdlog::debug!("Starting parsing.");

        while !self.is_at_end() {
            match self.declaration() {
                Some(statement) => statements.push(statement),
                None => {
                    App::error(
                        self.tokens[self.current].line,
                        "Failed to parse statement".to_string(),
                    );
                    self.synchronize();
                }
            }
        }
        statements
    }

    /// Parses declarations
    pub fn declaration(&mut self) -> Option<Stmt> {
        spdlog::debug!("parsing a declaration.");
        if self.match_token(vec![TokenType::Let]) {
            return self.let_declaration();
        }

        self.statement()
    }

    /// Parses let type of declarations
    pub fn let_declaration(&mut self) -> Option<Stmt> {
        spdlog::debug!("parsing a Let declaration.");
        let name = match self.consume(TokenType::Identifier, "Expect variable name.".to_string()) {
            Some(name) => name,
            None => return None,
        }
        .to_owned();

        if self.match_token(vec![TokenType::Equal]) {
            if let Some(initialiser) = self.expression() {
                self.consume(
                    TokenType::Semicolon,
                    "Expected ';' after value.".to_string(),
                );
                return Some(Stmt::Let(Box::new(StmtLet { name, initialiser })));
            }
        } else {
            self.consume(
                TokenType::Semicolon,
                "Expected ';' after value.".to_string(),
            );
            let initialiser = Expr::Literal(Box::new(ExprLiteral {
                value: TokenLiterals::Null,
            }));
            return Some(Stmt::Let(Box::new(StmtLet { name, initialiser })));
        }

        None
    }

    /// Parses statement if not a declaration.
    pub fn statement(&mut self) -> Option<Stmt> {
        spdlog::debug!("parsing a statement.");
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    /// parses print type of statement
    pub fn print_statement(&mut self) -> Option<Stmt> {
        spdlog::debug!("parsing a Let declaration.");
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after value.".to_string(),
        );
        if let Some(expr) = expr {
            return Some(Stmt::Print(Box::new(StmtPrint { expr })));
        }

        None
    }

    // parses expression type of statement.
    pub fn expression_statement(&mut self) -> Option<Stmt> {
        spdlog::debug!("parsing a expression statement.");
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression.".to_string(),
        );

        if let Some(expr) = expr {
            return Some(Stmt::Expr(Box::new(StmtExpr { expr })));
        }

        None
    }

    /// Parsing method for expressions.
    /// Nonterminal type.
    pub fn expression(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing expression");
        self.equality()
    }

    /// Parsing method for equality type expressions.
    /// Terminal type.
    pub fn equality(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing equality");
        if let Some(left) = self.comparison() {
            // recursively loop as long as we recieve BangEqual or EqualEqual type tokens.
            while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                let operator = self.previous().clone();
                if let Some(right) = self.comparison() {
                    return Some(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }
            return Some(left);
        }
        None
    }

    /// Parsing method for comparison type expressions.
    /// Nonterminal type.
    pub fn comparison(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing comparison");
        if let Some(left) = self.term() {
            // recursively loop as long as we recieve Greater, GreaterEqual,
            // Less, LessEqual type tokens.
            while self.match_token(vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]) {
                let operator = self.previous().clone();
                if let Some(right) = self.term() {
                    return Some(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Some(left);
        }

        None
    }

    /// Parsing method for term type expressions.
    /// Nonterminal type.
    pub fn term(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing term");
        if let Some(left) = self.factor() {
            // recursive loop as long as we recieve Minus or Plus type tokens.
            while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
                let operator = self.previous().clone();
                if let Some(right) = self.factor() {
                    return Some(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Some(left);
        }

        None
    }

    /// Parsing method for factor type expressions.
    /// Nonterminal type.
    pub fn factor(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing factor");
        if let Some(left) = self.unary() {
            while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
                let operator = self.previous().clone();
                if let Some(right) = self.unary() {
                    return Some(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Some(left);
        }
        None
    }

    /// Parsing method for unary type expressions.
    /// Nonterminal type.
    pub fn unary(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing unary");
        // if the expression is unary, recurisvely parse it.
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            if let Some(right) = self.unary() {
                return Some(Expr::Unary(Box::new(ExprUnary { operator, right })));
            }
        }

        // or return the primary of the unary expression.
        self.primary()
    }

    /// Parsing method for primary type expressions.
    /// Terminal type.
    pub fn primary(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing and matching primary");
        // false type token.
        if self.match_token(vec![TokenType::False]) {
            spdlog::trace!("matched literal: False");
            return Some(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Boolean(false),
            })));
        }

        // true type token.
        if self.match_token(vec![TokenType::True]) {
            spdlog::trace!("matched literal: True");
            return Some(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Boolean(true),
            })));
        }

        // null type token.
        if self.match_token(vec![TokenType::Null]) {
            spdlog::trace!("matched literal: Null");
            return Some(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Null,
            })));
        }

        // string value tokens.
        if self.match_token(vec![TokenType::String]) {
            spdlog::trace!("matched literal: String");
            return Some(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::String(self.previous().lexeme.to_owned()),
            })));
        }

        // number value tokens.
        if self.match_token(vec![TokenType::Number]) {
            spdlog::trace!("matched literal: Number");
            return Some(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Number(match self.previous().literal {
                    token::token_main::TokenLiterals::Number(v) => v,
                    _ => 0_f64,
                }),
            })));
        }

        // variable indentifiers.
        if self.match_token(vec![TokenType::Identifier]) {
            spdlog::trace!("matched literal: Identifier");
            return Some(Expr::Variable(Box::new(ExprVariable {
                name: self.previous().to_owned(),
            })));
        }

        // grouping.
        if self.match_token(vec![TokenType::LeftParen]) {
            spdlog::trace!("matched literal: LeftParen, trying to form a grouping.");
            if let Some(expr) = self.expression() {
                self.consume(
                    TokenType::RightParen,
                    "Expected ')' after expression".to_string(),
                );
                return Some(Expr::Grouping(Box::new(ExprGrouping { expression: expr })));
            }
        }

        self.parser_report_error(self.peek(), "Expected expression.".to_string());

        None
    }
}
