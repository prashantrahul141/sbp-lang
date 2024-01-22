use super::parser_main::Visitor;
use crate::parser::parser_main::walk_expr;

// to print the ast of parsed expressions.
pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
}

// implementing visitor for AstPrinter.
impl Visitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &super::parser_main::Binary) {
        print!("( {} ", expr.operator.lexeme);
        walk_expr(self, &expr.left);
        walk_expr(self, &expr.right);
        print!(" )");
    }

    fn visit_grouping_expr(&mut self, expr: &super::parser_main::Grouping) {
        print!("( group ");
        walk_expr(self, &expr.expression);
        print!(" )");
    }

    fn visit_literal_expr(&mut self, expr: &super::parser_main::Literal) {
        print!("{}", expr.value);
    }

    fn visit_unary_expr(&mut self, expr: &super::parser_main::Unary) {
        print!("( {} ", expr.operator.lexeme);
        walk_expr(self, &expr.right);
        print!(" )");
    }
}
