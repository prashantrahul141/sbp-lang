use super::interpreter_main::Interpreter;
use crate::{
    ast::stmt_ast::{walk_stmt, Stmt},
    token::token_main::TokenLiterals,
};

impl Interpreter {
    /// Constructor for Interpreter.
    pub fn new() -> Self {
        Self
    }

    /// top level public method to start interpretion of an expression.
    /// # Arguments
    /// * `expr` - The expression to interpret.
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        spdlog::info!("start interpreting");
        for statement in statements {
            self.execute(statement);
        }
    }

    pub fn execute(&mut self, statement: Stmt) {
        walk_stmt(self, &statement);
    }

    /// Splax core logic which defines what is truth?
    /// # Arguments
    /// * `token_literal` - Token literal to check if its truth or false.
    pub fn is_truth(&self, token_literal: TokenLiterals) -> bool {
        spdlog::trace!("checking truthy for literal : {}", token_literal);
        match token_literal {
            // any boolean is just it's value.
            TokenLiterals::Boolean(n) => n,
            // any number is true except 0.
            TokenLiterals::Number(value) => value != 0_f64,
            // any null type token literal are straigth up false.
            TokenLiterals::Null => false,
            // any string is true unless is empty.
            TokenLiterals::String(value) => !value.is_empty(),
        }
    }
}
