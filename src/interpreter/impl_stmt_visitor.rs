use super::interpreter_main::Interpreter;
use crate::ast::{expr_ast::walk_expr, stmt_ast::StmtVisitor};

/// Impl StmtVisitor pattern for Interpreter.
impl StmtVisitor for Interpreter {
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
}
