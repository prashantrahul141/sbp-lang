use crate::{
    app::app_main::App,
    token::token_main::{Token, TokenLiterals},
};
use std::collections::HashMap;

// Top level 'Environment' Data structure to store state of the interpreter.
pub struct Environment {
    pub values: HashMap<String, TokenLiterals>,
}

impl Environment {
    // constructor.
    pub fn new() -> Self {
        spdlog::debug!("creating new environment hashmap.");
        Self {
            values: HashMap::new(),
        }
    }

    /// Defines new variable in environment.
    /// # Arguments
    /// * `name` - String name of the variable.
    /// * `value` - Literal value of the variable in form of token literal,
    pub fn define(&mut self, name: String, value: TokenLiterals) {
        spdlog::debug!("defining variable with name : {name} and value : {value}");
        self.values.insert(name, value);
    }

    /// Assigns value to an already existing entry in the environment
    /// and return its value.
    /// Otherwise results panic error if the binding does not exist.
    /// # Arguments
    /// * `name` - String name of the variable.
    /// * `value` - Assignment value.
    pub fn assign(&mut self, name: Token, value: TokenLiterals) -> Option<TokenLiterals> {
        spdlog::debug!("Assigning value of '{name}' to '{value}'");
        if self.values.contains_key(&name.lexeme) {
            return self.values.insert(name.lexeme.to_string(), value);
        }

        App::runtime_error(
            name.line,
            format!("Reference to undefined variable '{}'", name.lexeme),
        );
        panic!();
    }

    /// Retrieves variable values from the environment, throws runtime error if not found.
    /// # Arguments
    /// * `name` - The token whose's lexeme value will be searched for.
    pub fn get(&self, name: Token) -> TokenLiterals {
        spdlog::debug!("finding variable with name : {name}");
        match self.values.get(&name.lexeme) {
            Some(value) => value.to_owned(),
            None => {
                App::runtime_error(
                    name.line,
                    format!("Reference to undefined variable '{}'", name.lexeme),
                );
                panic!();
            }
        }
    }
}
