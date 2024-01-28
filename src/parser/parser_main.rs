use super::error::ParserError;
use crate::{
    app::app_main::App,
    ast::{
        expr_ast::{
            Expr, ExprAssign, ExprBinary, ExprCall, ExprGrouping, ExprLiteral, ExprLogical,
            ExprUnary, ExprVariable,
        },
        stmt_ast::{Stmt, StmtBlock, StmtExpr, StmtFunc, StmtIf, StmtLet, StmtPrint, StmtWhile},
    },
    token::{
        self,
        token_main::{Token, TokenLiterals},
        token_types::TokenType,
    },
};
use std::vec;

/// Top level parser struct.
pub struct Parser {
    // vector of tokens to parse.
    pub tokens: Vec<Token>,
    // current progress of parsing.
    pub current: usize,
    // stores if there were any parsing error.
    pub has_error: bool,
}

impl Parser {
    /// Top level method which handles all parsing and returns vec
    /// of statements. Skips the statements which found to be faulty,
    /// while giving user errors.
    pub fn parse(&mut self) -> Vec<Stmt> {
        spdlog::debug!("Starting parsing.");
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    App::error_token(err.token, err.message);

                    // panic!
                    self.has_error = true;
                    self.synchronize();
                }
            }
        }

        statements
    }

    /// Parses declarations
    pub fn declaration(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("parsing a declaration.");
        if self.match_token(vec![TokenType::Fn]) {
            return self.fn_declaration();
        }
        if self.match_token(vec![TokenType::Let]) {
            return self.let_declaration();
        }

        self.statement()
    }

    /// parses fn type of declarations.
    pub fn fn_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = match self.consume(TokenType::Identifier, "Expected function name.".to_string())
        {
            Some(name) => name,
            None => {
                return Err(ParserError::new(
                    &self.tokens[self.current],
                    "Expected function name.".to_string(),
                ))
            }
        }
        .clone();

        self.consume(
            TokenType::LeftParen,
            "Expected '(' after function name.".to_string(),
        );

        let mut parameters: Vec<Token> = vec![];
        // if there are parameters.
        if !self.check(&TokenType::RightParen) {
            // rust's way of doing do-while loop.

            loop {
                if parameters.len() > 255 {
                    return Err(ParserError::new(
                        self.peek(),
                        "Can't have more than 255 function parameters.".to_string(),
                    ));
                };

                if let Some(param) = self.consume(
                    TokenType::Identifier,
                    "Expected parameter name. ".to_string(),
                ) {
                    parameters.push(param.to_owned());
                };

                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        // finnaly consuming closing ')'
        self.consume(
            TokenType::RightParen,
            "Expected ')' after parameters.".to_string(),
        );

        // now parsing function body
        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after function signature.".to_string(),
        );

        let body = self.block();

        Ok(Stmt::Function(Box::new(StmtFunc {
            name: name.to_owned(),
            body: StmtBlock {
                block_statements: body,
            },
            params: parameters,
        })))
    }

    /// Parses let type of
    pub fn let_declaration(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("parsing a Let declaration.");
        let name = match self.consume(
            TokenType::Identifier,
            "Expectecd variable name.".to_string(),
        ) {
            Some(name) => name,
            None => {
                return Err(ParserError::new(
                    &self.tokens[self.current],
                    "Expected variable name.".to_string(),
                ))
            }
        }
        .to_owned();

        if self.match_token(vec![TokenType::Equal]) {
            if let Ok(initialiser) = self.expression() {
                self.consume(
                    TokenType::Semicolon,
                    "Expected ';' after value.".to_string(),
                );
                return Ok(Stmt::Let(Box::new(StmtLet { name, initialiser })));
            }
        } else {
            self.consume(
                TokenType::Semicolon,
                "Expected ';' after value.".to_string(),
            );
            let initialiser = Expr::Literal(Box::new(ExprLiteral {
                value: TokenLiterals::Null,
            }));
            return Ok(Stmt::Let(Box::new(StmtLet { name, initialiser })));
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse let declaration".to_string(),
        ))
    }

    /// Parses statement if not a declaration.
    pub fn statement(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("parsing a statement.");

        // while print indentifier is found.
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }

        // while left brace is found.
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Box::new(StmtBlock {
                block_statements: self.block(),
            })));
        }

        // for loops

        if self.match_token(vec![TokenType::For]) {
            return self.for_statement();
        }

        // while if indentifier is found.
        if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        }

        // while while indentifier is found.
        if self.match_token(vec![TokenType::While]) {
            return self.while_statement();
        }

        self.expression_statement()
    }

    /// desugars for loop
    pub fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("desugaring a for loop stmt.");

        // starting left paren
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'for' ".to_string(),
        );

        // parsing initialiser
        let initialiser: Result<Stmt, ParserError>;
        if self.match_token(vec![TokenType::Semicolon]) {
            initialiser = Err(ParserError::new(
                self.previous(),
                "Found semicolon before initialiser".to_string(),
            ));
        } else if self.match_token(vec![TokenType::Let]) {
            initialiser = self.let_declaration();
        } else {
            initialiser = self.expression_statement();
        }

        // parsing loop condition.
        let mut condition: Result<Expr, ParserError> = Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse loop condition.".to_string(),
        ));
        if !self.check(&TokenType::Semicolon) {
            condition = self.expression();
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after loop condition".to_string(),
        );

        // parsing loop incrementer.
        let mut increment: Result<Expr, ParserError> = Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse loop incrementer.".to_string(),
        ));
        if !self.check(&TokenType::RightParen) {
            increment = self.expression();
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after loop increment".to_string(),
        );

        // creating new body
        if let Ok(mut body) = self.statement() {
            // if there is a increment.

            if let Ok(increment) = increment {
                body = Stmt::Block(Box::new(StmtBlock {
                    block_statements: vec![
                        body,
                        Stmt::Expr(Box::new(StmtExpr { expr: increment })),
                    ],
                }));
            }

            // if there are no condition, we default to true.
            if condition.is_err() {
                condition = Ok(Expr::Literal(Box::new(ExprLiteral {
                    value: TokenLiterals::Boolean(true),
                })));
            }

            // create while loop with the above condition and parsed body.
            body = Stmt::While(Box::new(StmtWhile {
                // we already checked if condition was None, so its safe to assume
                // its going to be non None here.
                condition: condition?,
                body,
            }));

            // if there is a initialiser, we add it before the while loop.
            if let Ok(initialiser) = initialiser {
                body = Stmt::Block(Box::new(StmtBlock {
                    block_statements: vec![initialiser, body],
                }));
            }

            return Ok(body);
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse for statement.".to_string(),
        ))
    }

    /// parses while type of statement.
    pub fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'while'".to_string(),
        );

        let condition = match self.expression() {
            Ok(condition) => condition,
            Err(e) => return Err(e),
        };

        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition".to_string(),
        );

        let body = match self.statement() {
            Ok(body) => body,
            Err(e) => return Err(e),
        };

        Ok(Stmt::While(Box::new(StmtWhile { condition, body })))
    }

    /// parses if type of statement
    pub fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LeftParen, "expected '(' after 'if'".to_string());
        // the condition inside 'if ()'
        let condition = match self.expression() {
            Ok(condition) => condition,
            Err(e) => return Err(e),
        };

        self.consume(
            TokenType::RightParen,
            "expected ')' after 'condition'".to_string(),
        );

        // block inside if condition tree.

        let then_branch = match self.statement() {
            Ok(statement) => statement,
            Err(e) => return Err(e),
        };

        // optional else branch.
        let mut else_branch: Result<Stmt, ParserError> = Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse optional else branch".to_string(),
        ));

        if self.match_token(vec![TokenType::Else]) {
            else_branch = self.statement();
        }

        Ok(Stmt::If(Box::new(StmtIf {
            condition,
            then_branch,
            else_branch,
        })))
    }

    /// parses print type of statement
    pub fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("parsing a Let declaration.");
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after value.".to_string(),
        );
        if let Ok(expr) = expr {
            return Ok(Stmt::Print(Box::new(StmtPrint { expr })));
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse print statement.".to_string(),
        ))
    }

    pub fn block(&mut self) -> Vec<Stmt> {
        let mut block_statements = vec![];
        while !self.match_token(vec![TokenType::RightBrace]) && !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => block_statements.push(stmt),
                Err(e) => {
                    let current = self.tokens[self.current].clone();
                    self.parser_report_error(
                        &current,
                        format!("Failed parsing block statements because declaration failed due to : {e}")
                    );
                }
            }
        }

        block_statements
    }

    // parses expression type of statement.
    pub fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        spdlog::debug!("parsing a expression statement.");
        let expr = self.expression();
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after expression.".to_string(),
        );

        if let Ok(expr) = expr {
            return Ok(Stmt::Expr(Box::new(StmtExpr { expr })));
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse expression statement.".to_string(),
        ))
    }

    /// Parsing method for expressions.
    /// Nonterminal type.
    pub fn expression(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing expression");
        self.assignment()
    }

    /// Parsing method for assignment expressions
    pub fn assignment(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing assignment");
        let expr = match self.or() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        // if we find a '='.
        if self.match_token(vec![TokenType::Equal]) {
            if let Ok(value) = self.assignment() {
                if let Expr::Variable(expr) = expr {
                    // create assignment expression if left token was variable,
                    // with a equal,
                    // with a valid expression/assignment on right.
                    let name = expr.name;
                    return Ok(Expr::Assignment(Box::new(ExprAssign { name, value })));
                }
            }

            // we error if found weird assignment expression.
            App::runtime_error(self.previous().line, "Invalid assignment.".to_string());
            panic!();
        }

        // return the expr itself if didnt found a '='
        Ok(expr)
    }

    // Parsing logical or
    pub fn or(&mut self) -> Result<Expr, ParserError> {
        if let Ok(left) = self.and() {
            // recursively loop as long as we recieve OR type tokens.
            while self.match_token(vec![TokenType::Or]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.and() {
                    return Ok(Expr::Logical(Box::new(ExprLogical {
                        left,
                        operator,
                        right,
                    })));
                }
            }
            return Ok(left);
        }
        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse logical 'or'".to_string(),
        ))
    }

    // parsing logical and
    pub fn and(&mut self) -> Result<Expr, ParserError> {
        if let Ok(left) = self.equality() {
            // recursively loop as long as we recieve AND type tokens.
            while self.match_token(vec![TokenType::And]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.equality() {
                    return Ok(Expr::Logical(Box::new(ExprLogical {
                        left,
                        operator,
                        right,
                    })));
                }
            }
            return Ok(left);
        }
        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse logical 'and'".to_string(),
        ))
    }

    /// Parsing method for equality type expressions.
    /// Terminal type.
    pub fn equality(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing equality");
        if let Ok(left) = self.comparison() {
            // recursively loop as long as we recieve BangEqual or EqualEqual type tokens.
            while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.comparison() {
                    return Ok(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }
            return Ok(left);
        }
        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse 'equality'".to_string(),
        ))
    }

    /// Parsing method for comparison type expressions.
    /// Nonterminal type.
    pub fn comparison(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing comparison");
        if let Ok(left) = self.term() {
            // recursively loop as long as we recieve Greater, GreaterEqual,
            // Less, LessEqual type tokens.
            while self.match_token(vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.term() {
                    return Ok(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Ok(left);
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse comparison".to_string(),
        ))
    }

    /// Parsing method for term type expressions.
    /// Nonterminal type.
    pub fn term(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing term");
        if let Ok(left) = self.factor() {
            // recursive loop as long as we recieve Minus or Plus type tokens.
            while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.factor() {
                    return Ok(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Ok(left);
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse term".to_string(),
        ))
    }

    /// Parsing method for factor type expressions.
    /// Nonterminal type.
    pub fn factor(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing factor");
        if let Ok(left) = self.unary() {
            while self.match_token(vec![TokenType::Slash, TokenType::Star, TokenType::Mod]) {
                let operator = self.previous().clone();
                if let Ok(right) = self.unary() {
                    return Ok(Expr::Binary(Box::new(ExprBinary {
                        left,
                        operator,
                        right,
                    })));
                }
            }

            return Ok(left);
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed to parse factor".to_string(),
        ))
    }

    /// Parsing method for unary type expressions.
    /// Nonterminal type.
    pub fn unary(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing unary");
        // if the expression is unary, recurisvely parse it.
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            if let Ok(right) = self.unary() {
                return Ok(Expr::Unary(Box::new(ExprUnary { operator, right })));
            }
        }

        // the expression might be a call.
        self.call()
    }

    // Parsing function call.
    pub fn call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.primary();
        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }

        expr
    }
    // parsing tailing function calls.
    pub fn finish_call(&mut self, callee: Result<Expr, ParserError>) -> Result<Expr, ParserError> {
        if let Ok(callee) = callee {
            let mut arguments = vec![];

            // if we find right paren.
            if !self.check(&TokenType::RightParen) {
                // rust way of doing do-while loop.
                loop {
                    if arguments.len() >= 255 {
                        let error_token = &self.tokens[self.current].to_owned();
                        self.parser_report_error(
                            error_token,
                            "Can't have more than 255 function arguments.".to_string(),
                        )
                    }

                    // add arguments.
                    if let Ok(argument) = self.expression() {
                        arguments.push(argument);
                    }
                    // if we see a comma we have reached the end of this argument.
                    if !self.match_token(vec![TokenType::Comma]) {
                        break;
                    }
                }
            }

            if let Some(paren) = self.consume(
                TokenType::RightParen,
                "Expected ')' after function arguments.".to_string(),
            ) {
                return Ok(Expr::Call(Box::new(ExprCall {
                    callee,
                    paren: paren.to_owned(),
                    arguments,
                })));
            }
        }
        Err(ParserError::new(
            &self.tokens[self.current],
            "Failed parsing functon call.".to_string(),
        ))
    }

    /// Parsing method for primary type expressions.
    /// Terminal type.
    pub fn primary(&mut self) -> Result<Expr, ParserError> {
        spdlog::trace!("parsing and matching primary");
        // false type token.
        if self.match_token(vec![TokenType::False]) {
            spdlog::trace!("matched literal: False");
            return Ok(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Boolean(false),
            })));
        }

        // true type token.
        if self.match_token(vec![TokenType::True]) {
            spdlog::trace!("matched literal: True");
            return Ok(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Boolean(true),
            })));
        }

        // null type token.
        if self.match_token(vec![TokenType::Null]) {
            spdlog::trace!("matched literal: Null");
            return Ok(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Null,
            })));
        }

        // string value tokens.
        if self.match_token(vec![TokenType::String]) {
            spdlog::trace!("matched literal: String");
            return Ok(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::String(self.previous().lexeme.to_owned()),
            })));
        }

        // number value tokens.
        if self.match_token(vec![TokenType::Number]) {
            spdlog::trace!("matched literal: Number");
            return Ok(Expr::Literal(Box::new(ExprLiteral {
                value: token::token_main::TokenLiterals::Number(match self.previous().literal {
                    token::token_main::TokenLiterals::Number(v) => v,
                    _ => 0_f64,
                }),
            })));
        }

        // variable indentifiers.
        if self.match_token(vec![TokenType::Identifier]) {
            spdlog::trace!("matched literal: Identifier");
            return Ok(Expr::Variable(Box::new(ExprVariable {
                name: self.previous().to_owned(),
            })));
        }

        // grouping.
        if self.match_token(vec![TokenType::LeftParen]) {
            spdlog::trace!("matched literal: LeftParen, trying to form a grouping.");
            if let Ok(expr) = self.expression() {
                self.consume(
                    TokenType::RightParen,
                    "Expected ')' after expression".to_string(),
                );
                return Ok(Expr::Grouping(Box::new(ExprGrouping { expression: expr })));
            }
        }

        Err(ParserError::new(
            &self.tokens[self.current],
            "Expected expression.".to_string(),
        ))
    }
}
