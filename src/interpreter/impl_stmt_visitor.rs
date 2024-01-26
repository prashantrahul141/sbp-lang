use super::{environment::Environment, interpreter_main::Interpreter};
use crate::ast::{expr_ast::walk_expr, stmt_ast::StmtVisitor};

/// Impl StmtVisitor pattern for Interpreter.
impl StmtVisitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtBlock) {
        self.execute_block(
            stmt,
            Environment::new(Some(Box::new(self.environment.clone()))),
        );
    }

    /// struct method walks/executes expression statements.
    /// # Arguments
    /// * `stmt` - stmtexpr stmt to walk.
    fn visit_expression_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtExpr) {
        spdlog::debug!("evaluating expression stmt: {:?}", stmt);
        walk_expr(self, &stmt.expr);
    }

    /// struct method walks/executes print statements.
    /// # Arguments
    /// * `stmt` - stmtprint stmt to walk.
    fn visit_print_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtPrint) {
        spdlog::debug!("evaluating print stmt: {:?}", stmt);
        let value = walk_expr(self, &stmt.expr);
        println!("{}", value);
    }

    /// struct method walks/executes let statements.
    /// # Arguments
    /// * `stmt` - stmtlet stmt to walk.
    fn visit_let_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtLet) {
        let value = walk_expr(self, &stmt.initialiser);
        spdlog::debug!(
            "defining variable : {} with value : {}",
            stmt.name.lexeme,
            value
        );
        self.environment.define(stmt.name.lexeme.to_owned(), value);
    }
}
