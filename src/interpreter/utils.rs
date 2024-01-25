use super::interpreter_main::Interpreter;
use crate::{
    ast::ast_tree::{walk_expr, Expr},
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
    pub fn interpret(&mut self, expr: Expr) -> TokenLiterals {
        spdlog::info!("start interpreting");
        walk_expr(self, &expr)
    }

    /// Splax core logic which defines what is truth?
    /// # Arguments
    /// * `token_literal` - Token literal to check if its truth or false.
    pub fn is_truth(&self, token_literal: TokenLiterals) -> TokenLiterals {
        spdlog::trace!("checking truthy for literal : {}", token_literal);
        match token_literal {
            // any boolean is just it's value.
            TokenLiterals::Boolean(_) => token_literal,
            // any number is true except 0.
            TokenLiterals::Number(value) => TokenLiterals::Boolean(value != 0_f64),
            // any null type token literal are straigth up false.
            TokenLiterals::Null => TokenLiterals::Boolean(false),
            // any string is true unless is empty.
            TokenLiterals::String(value) => TokenLiterals::Boolean(!value.is_empty()),
        }
    }
}
