use crate::token::token_main::{Token, TokenLiterals};

/// Base Expression enum.
/// Holds variants for all types of expressions.
#[derive(Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

/// display implementation for token expr.
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(n) => write!(f, "{}", n),
            Expr::Grouping(n) => write!(f, "{}", n),
            Expr::Literal(n) => write!(f, "{}", n),
            Expr::Unary(n) => write!(f, "{}", n),
        }
    }
}

/// Visitor trait.
/// Since we will be using visitor pattern for the expressions.
///
/// When any new pass/feature we need to implement to the expressions,
/// we just impl this visitor trait to that struct.
pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &Binary);
    fn visit_grouping_expr(&mut self, expr: &Grouping);
    fn visit_literal_expr(&mut self, expr: &Literal);
    fn visit_unary_expr(&mut self, expr: &Unary);
}

/// Walker, in other implementation this will be called `accept`.
/// # Arguments
/// * `visitor` - The visitor struct which implements Visitor trait.
/// * `expr` - The expression to walk.
pub fn walk_expr(visitor: &mut dyn Visitor, expr: &Expr) {
    match expr {
        Expr::Binary(e) => visitor.visit_binary_expr(e),
        Expr::Grouping(e) => visitor.visit_grouping_expr(e),
        Expr::Literal(e) => visitor.visit_literal_expr(e),
        Expr::Unary(e) => visitor.visit_unary_expr(e),
    };
}

/// Grammer for binary expressions.
#[derive(Debug)]
pub struct Binary {
    // left operand.
    pub left: Expr,
    // operator.
    pub operator: Token,
    // right operand.
    pub right: Expr,
}

/// display implementation for binary token.
impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.operator.lexeme, self.left, self.right)
    }
}

/// Grammer for grouping expressions.
#[derive(Debug)]
pub struct Grouping {
    // grouped expression.
    pub expression: Expr,
}

/// display implementation for grouping token.
impl std::fmt::Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( group {} )", self.expression)
    }
}

/// Grammer for literals.ast
#[derive(Debug)]
pub struct Literal {
    // token literal.
    pub value: TokenLiterals,
}

/// display implementation for literal token.
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Grammer for unary expressions.
#[derive(Debug)]
pub struct Unary {
    /// unary operator.
    pub operator: Token,
    /// operand.
    pub right: Expr,
}

/// display implementation for unary token.
impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.operator, self.right)
    }
}
