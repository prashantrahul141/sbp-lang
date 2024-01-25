use super::interpreter_main::Interpreter;
use crate::ast::{expr_ast::walk_expr, stmt_ast::StmtVisitor};

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtExpr) {
        walk_expr(self, &stmt.expr);
    }

    fn visit_print_stmt(&mut self, stmt: &crate::ast::stmt_ast::StmtPrint) {
        let value = walk_expr(self, &stmt.expr);
        println!("{}", value);
    }
}
