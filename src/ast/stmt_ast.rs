use super::expr_ast::Expr;

/// Top level statements enum.
#[derive(Debug)]
pub enum Stmt {
    Expr(Box<StmtExpr>),
    Print(Box<StmtPrint>),
}

/// Visitor trait for statements.
/// Since we will be using visitor pattern for the statements.
///
/// When any new pass/feature we need to implement to the statements,
/// we just impl this visitor trait to that struct.
pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &StmtExpr);
    fn visit_print_stmt(&mut self, stmt: &StmtPrint);
}

/// Walker, in other implementation this will be called `accept`.
/// # Arguments
/// * `visitor` - The visitor struct which implements StmtVisitor trait.
/// * `stmt` - The stmt to walk.
pub fn walk_stmt(visitor: &mut dyn StmtVisitor, stmt: &Stmt) {
    match stmt {
        Stmt::Expr(stmt) => visitor.visit_expression_stmt(stmt),
        Stmt::Print(stmt) => visitor.visit_print_stmt(stmt),
    }
}

/// Grammer for stmtexpr statemments.
#[derive(Debug)]
pub struct StmtExpr {
    pub expr: Expr,
}

/// Grammer for stmtprint statemments.
#[derive(Debug)]
pub struct StmtPrint {
    pub expr: Expr,
}
