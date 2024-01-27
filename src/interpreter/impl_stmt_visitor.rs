use super::{environment::Environment, interpreter_main::Interpreter};
use crate::ast::{self, expr_ast::walk_expr, stmt_ast::StmtVisitor};

/// Impl StmtVisitor pattern for Interpreter.
impl StmtVisitor for Interpreter {
    // recursively executes a block of statements.
    fn visit_block_stmt(&mut self, stmt: &ast::stmt_ast::StmtBlock) {
        self.execute_block(
            stmt,
            Box::new(Environment::new(Some(self.environment.clone()))),
        );
    }

    /// struct method walks/executes expression statements.
    /// # Arguments
    /// * `stmt` - stmtexpr stmt to walk.
    fn visit_expression_stmt(&mut self, stmt: &ast::stmt_ast::StmtExpr) {
        spdlog::debug!("evaluating expression stmt: {:?}", stmt);
        walk_expr(self, &stmt.expr);
    }

    /// struct method walks/executes print statements.
    /// # Arguments
    /// * `stmt` - stmtprint stmt to walk.
    fn visit_print_stmt(&mut self, stmt: &ast::stmt_ast::StmtPrint) {
        spdlog::debug!("evaluating print stmt: {:?}", stmt);
        let value = walk_expr(self, &stmt.expr);
        println!("{}", value);
    }

    /// struct method walks/executes let statements.
    /// # Arguments
    /// * `stmt` - stmtlet stmt to walk.
    fn visit_let_stmt(&mut self, stmt: &ast::stmt_ast::StmtLet) {
        let value = walk_expr(self, &stmt.initialiser);
        spdlog::debug!(
            "defining variable : {} with value : {}",
            stmt.name.lexeme,
            value
        );
        self.environment.define(stmt.name.lexeme.to_owned(), value);
    }

    /// method walks/executes if statements.
    /// # Arguments
    /// * `stmt` - stmtif stmt to walk.
    fn visit_if_stmt(&mut self, stmt: &ast::stmt_ast::StmtIf) {
        // evaluate condition into token literal.
        let evaluated_condition = walk_expr(self, &stmt.condition);
        spdlog::debug!("executing if block, evaluated condition : {evaluated_condition}");

        // check truthy for the token literal.
        if Interpreter::is_truth(evaluated_condition) {
            spdlog::trace!("executing then branch.");
            // execute then block if token literal is truthy
            self.execute(&stmt.then_branch);
        } else {
            spdlog::trace!("checking and executing else branch.");
            // else check if `else_branch` exists on the stmt, and execute it.
            if let Some(else_branch) = &stmt.else_branch {
                self.execute(else_branch);
            }
        }
    }

    /// method walks/executes while statements.
    /// # Arguments
    /// * `stmt` - stmtwhile stmt to walk.
    fn visit_while_stmt(&mut self, stmt: &ast::stmt_ast::StmtWhile) {
        while Interpreter::is_truth(walk_expr(self, &stmt.condition)) {
            self.execute(&stmt.body);
        }
    }
}
