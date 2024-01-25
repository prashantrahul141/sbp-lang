use super::expr_ast::Expr;

/// Top level statements enum.
pub enum Stmt {
    Expr(Box<StmtExpr>),
    Print(Box<StmtPrint>),
}

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpr);
    fn visit_print_stmt(&mut self, stmt: &StmtPrint);
}

pub fn walk_stmt(visitor: &mut dyn StmtVisitor, stmt: &Stmt) {
    match stmt {
        Stmt::Expr(stmt) => visitor.visit_expression_stmt(stmt),
        Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
    }
}

pub struct StmtExpr {
    pub expr: Expr,
}

pub struct StmtPrint {
    pub expr: Expr,
}
