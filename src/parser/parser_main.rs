use std::vec;

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
        let mut statements = vec![];
        spdlog::debug!("Starting parsing.");

        while !self.is_at_end() {
            if let Some(statement) = self.declaration() {
                statements.push(statement);
                continue;
            }

            App::error(
                self.tokens[self.current].line,
                "Failed to parse statement".to_string(),
            );
            self.synchronize();
        }

        statements
    }

    /// Parses declarations
    pub fn declaration(&mut self) -> Option<Stmt> {
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
    pub fn fn_declaration(&mut self) -> Option<Stmt> {
        let name = match self.consume(TokenType::Identifier, "Expected function name.".to_string())
        {
            Some(name) => name,
            None => return None,
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
                    let peeked = self.peek().to_owned();
                    self.parser_report_error(
                        &peeked,
                        "Can't have more than 255 function parameters.".to_string(),
                    );
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

        Some(Stmt::Function(Box::new(StmtFunc {
            name: name.to_owned(),
            body: StmtBlock {
                block_statements: body,
            },
            params: parameters,
        })))
    }

    /// Parses let type of
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
        // while print indentifier is found.
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }

        // while left brace is found.
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Some(Stmt::Block(Box::new(StmtBlock {
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
    pub fn for_statement(&mut self) -> Option<Stmt> {
        spdlog::debug!("desugaring a for loop stmt.");

        // starting left paren
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'for' ".to_string(),
        );

        // parsing initialiser
        let initialiser: Option<Stmt>;
        if self.match_token(vec![TokenType::Semicolon]) {
            initialiser = None;
        } else if self.match_token(vec![TokenType::Let]) {
            initialiser = self.let_declaration();
        } else {
            initialiser = self.expression_statement();
        }

        // parsing loop condition.
        let mut condition: Option<Expr> = None;
        if !self.check(&TokenType::Semicolon) {
            condition = self.expression();
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after loop condition".to_string(),
        );

        // parsing loop incrementer.
        let mut increment: Option<Expr> = None;
        if !self.check(&TokenType::RightParen) {
            increment = self.expression();
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after loop increment".to_string(),
        );

        // creating new body
        if let Some(mut body) = self.statement() {
            // if there is a increment.

            if let Some(increment) = increment {
                body = Stmt::Block(Box::new(StmtBlock {
                    block_statements: vec![
                        body,
                        Stmt::Expr(Box::new(StmtExpr { expr: increment })),
                    ],
                }));
            }

            // if there are no condition, we default to true.
            if condition.is_none() {
                condition = Some(Expr::Literal(Box::new(ExprLiteral {
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
            if let Some(initialiser) = initialiser {
                body = Stmt::Block(Box::new(StmtBlock {
                    block_statements: vec![initialiser, body],
                }));
            }

            return Some(body);
        }

        None
    }

    /// parses while type of statement.
    pub fn while_statement(&mut self) -> Option<Stmt> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'while'".to_string(),
        );

        let condition = match self.expression() {
            Some(condition) => condition,
            None => return None,
        };

        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition".to_string(),
        );

        let body = match self.statement() {
            Some(body) => body,
            None => return None,
        };

        Some(Stmt::While(Box::new(StmtWhile { condition, body })))
    }

    /// parses if type of statement
    pub fn if_statement(&mut self) -> Option<Stmt> {
        self.consume(TokenType::LeftParen, "expected '(' after 'if'".to_string());
        // the condition inside 'if ()'
        let condition = match self.expression() {
            Some(condition) => condition,
            None => return None,
        };

        self.consume(
            TokenType::RightParen,
            "expected ')' after 'condition'".to_string(),
        );

        // block inside if condition tree.
        let then_branch = match self.statement() {
            Some(statement) => statement,
            None => return None,
        };

        // optional else branch.
        let mut else_branch: Option<Stmt> = None;
        if self.match_token(vec![TokenType::Else]) {
            else_branch = self.statement();
        }

        Some(Stmt::If(Box::new(StmtIf {
            condition,
            then_branch,
            else_branch,
        })))
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

    pub fn block(&mut self) -> Vec<Stmt> {
        let mut block_statements = vec![];
        while !self.match_token(vec![TokenType::RightBrace]) && !self.is_at_end() {
            match self.declaration() {
                Some(stmt) => block_statements.push(stmt),
                None => {
                    let current = self.tokens[self.current].clone();
                    self.parser_report_error(
                        &current,
                        "Failed parsing block statements.".to_string(),
                    );
                }
            }
        }

        block_statements
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
        self.assignment()
    }

    /// Parsing method for assignment expressions
    pub fn assignment(&mut self) -> Option<Expr> {
        spdlog::trace!("parsing assignment");
        let expr = match self.or() {
            Some(expr) => expr,
            None => return None,
        };

        // if we find a '='.
        if self.match_token(vec![TokenType::Equal]) {
            if let Some(value) = self.assignment() {
                if let Expr::Variable(expr) = expr {
                    // create assignment expression if left token was variable,
                    // with a equal,
                    // with a valid expression/assignment on right.
                    let name = expr.name;
                    return Some(Expr::Assignment(Box::new(ExprAssign { name, value })));
                }
            }

            // we error if found weird assignment expression.
            App::runtime_error(self.previous().line, "Invalid assignment.".to_string());
            panic!();
        }

        // return the expr itself if didnt found a '='
        Some(expr)
    }

    // Parsing logical or
    pub fn or(&mut self) -> Option<Expr> {
        if let Some(left) = self.and() {
            // recursively loop as long as we recieve OR type tokens.
            while self.match_token(vec![TokenType::Or]) {
                let operator = self.previous().clone();
                if let Some(right) = self.and() {
                    return Some(Expr::Logical(Box::new(ExprLogical {
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

    // parsing logical and
    pub fn and(&mut self) -> Option<Expr> {
        if let Some(left) = self.equality() {
            // recursively loop as long as we recieve AND type tokens.
            while self.match_token(vec![TokenType::And]) {
                let operator = self.previous().clone();
                if let Some(right) = self.equality() {
                    return Some(Expr::Logical(Box::new(ExprLogical {
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
            while self.match_token(vec![TokenType::Slash, TokenType::Star, TokenType::Mod]) {
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

        // the expression might be a call.
        self.call()
    }

    // Parsing function call.
    pub fn call(&mut self) -> Option<Expr> {
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
    pub fn finish_call(&mut self, callee: Option<Expr>) -> Option<Expr> {
        if let Some(callee) = callee {
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
                    if let Some(argument) = self.expression() {
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
                return Some(Expr::Call(Box::new(ExprCall {
                    callee,
                    paren: paren.to_owned(),
                    arguments,
                })));
            }
        }

        None
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

        let peek = self.peek().clone();
        self.parser_report_error(&peek, "Expected expression.".to_string());

        None
    }
}
