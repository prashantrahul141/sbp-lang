use super::{environment::Environment, interpreter_main::Interpreter};
use crate::{
    app::app_main::App,
    ast::stmt_ast::{walk_stmt, Stmt, StmtBlock},
    token::token_main::TokenLiterals,
};

impl Interpreter {
    /// Constructor for Interpreter.
    pub fn new() -> Self {
        spdlog::debug!("constructing new interpreter.");
        let globals = Interpreter::get_globals();
        // App::runtime_error(2, format!("{}", globals.get("__VERSION__")));
        Self {
            environment: globals.clone(),
            globals,
        }
    }

    /// top level public method to start interpretion of program.
    /// # Arguments
    /// * `statements` - A vector of statements aka a program.
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        spdlog::info!("start interpreting");
        for statement in statements {
            self.execute(&statement);
        }
    }

    /// Executes a block of statements, give it a new environment.
    /// # Arguments
    /// * `enclosing` - The enclosing environment.
    pub fn execute_block(
        &mut self,
        block_statements: &StmtBlock,
        child_environment: Box<Environment>,
    ) {
        // setting current env as child environment.
        self.environment = child_environment;

        // executing block statements.
        for stmt in &block_statements.block_statements {
            self.execute(stmt);
        }

        // take current's parent environment, clone it and make it current environment
        if let Some(parent_environment) = self.environment.enclosing.clone() {
            self.environment = parent_environment;
            return;
        }

        // this is unreachable, if you somehow manage to trigger it,
        // feel free to fix it and make a pr.
        App::runtime_error(0, "Reassignment of environment failed.".to_string());
    }

    /// Walks one statement at a time.
    pub fn execute(&mut self, statement: &Stmt) {
        spdlog::debug!("executing stmt : {:?}", statement);
        walk_stmt(self, statement);
    }

    /// Splax core logic which defines what is truth?
    /// # Arguments
    /// * `token_literal` - Token literal to check if its truth or false.
    pub fn is_truth(token_literal: TokenLiterals) -> bool {
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
