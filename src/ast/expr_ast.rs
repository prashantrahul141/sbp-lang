use crate::{
    app::app_main::App,
    interpreter::{
        environment::{self, Environment},
        interpreter_main::Interpreter,
    },
    token::token_main::{Token, TokenLiterals},
};

use super::stmt_ast::StmtFunc;

/// Base Expression enum.
/// Holds variants for all types of expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<ExprBinary>),
    Call(Box<ExprCall>),
    Grouping(Box<ExprGrouping>),
    Literal(Box<ExprLiteral>),
    Logical(Box<ExprLogical>),
    Unary(Box<ExprUnary>),
    Variable(Box<ExprVariable>),
    Assignment(Box<ExprAssign>),
}

/// display implementation for token expr.
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(n) => write!(f, "{}", n),
            Expr::Call(n) => write!(f, "{}({:?})", n.callee, n.arguments),
            Expr::Grouping(n) => write!(f, "{}", n),
            Expr::Literal(n) => write!(f, "{}", n),
            Expr::Unary(n) => write!(f, "{}", n),
            Expr::Variable(n) => write!(f, "{}", n.name),
            Expr::Assignment(n) => write!(f, "{} : {}", n.name, n.value),
            Expr::Logical(n) => write!(f, "{} {} {}", n.left, n.operator, n.right),
        }
    }
}

/// Visitor trait for expressions.
/// Since we will be using visitor pattern for the expressions.
///
/// When any new pass/feature we need to implement to the expressions,
/// we just impl this visitor trait to that struct.
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &ExprBinary) -> T;
    fn visit_call_expr(&mut self, expr: &ExprCall) -> T;
    fn visit_grouping_expr(&mut self, expr: &ExprGrouping) -> T;
    fn visit_literal_expr(&mut self, expr: &ExprLiteral) -> T;
    fn visit_unary_expr(&mut self, expr: &ExprUnary) -> T;
    fn visit_let_expr(&mut self, expr: &ExprVariable) -> T;
    fn visit_assign_expr(&mut self, expr: &ExprAssign) -> T;
    fn visit_logical_expr(&mut self, expr: &ExprLogical) -> T;
}

/// Walker, in other implementation this will be called `accept`.
/// # Arguments
/// * `visitor` - The visitor struct which implements Visitor trait.
/// * `expr` - The expression to walk.
pub fn walk_expr<T>(visitor: &mut dyn ExprVisitor<T>, expr: &Expr) -> T {
    match expr {
        Expr::Binary(e) => visitor.visit_binary_expr(e),
        Expr::Grouping(e) => visitor.visit_grouping_expr(e),
        Expr::Literal(e) => visitor.visit_literal_expr(e),
        Expr::Unary(e) => visitor.visit_unary_expr(e),
        Expr::Variable(e) => visitor.visit_let_expr(e),
        Expr::Assignment(e) => visitor.visit_assign_expr(e),
        Expr::Logical(e) => visitor.visit_logical_expr(e),
        Expr::Call(e) => visitor.visit_call_expr(e),
    }
}

/// Grammer for binary expressions.
#[derive(Debug, Clone)]
pub struct ExprBinary {
    // left operand.
    pub left: Expr,
    // operator.
    pub operator: Token,
    // right operand.
    pub right: Expr,
}

/// display implementation for binary token.
impl std::fmt::Display for ExprBinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.operator.lexeme, self.left, self.right)
    }
}

/// Grammer for grouping expressions.
#[derive(Debug, Clone)]
pub struct ExprGrouping {
    // grouped expression.
    pub expression: Expr,
}

/// display implementation for grouping token.
impl std::fmt::Display for ExprGrouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( group {} )", self.expression)
    }
}

/// Grammer for literals.ast
#[derive(Debug, Clone)]
pub struct ExprLiteral {
    // token literal.
    pub value: TokenLiterals,
}

/// display implementation for literal token.
impl std::fmt::Display for ExprLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Grammer for unary expressions.
#[derive(Debug, Clone)]
pub struct ExprUnary {
    /// unary operator.
    pub operator: Token,
    /// operand.
    pub right: Expr,
}

/// display implementation for unary token.
impl std::fmt::Display for ExprUnary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.operator, self.right)
    }
}

/// Grammer for variable declarations.
#[derive(Debug, Clone)]
pub struct ExprVariable {
    // name of the variable.
    pub name: Token,
}

/// display implementation for unary token.
impl std::fmt::Display for ExprVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Grammer for assignment expressions.
#[derive(Debug, Clone)]
pub struct ExprAssign {
    // name of the variable
    pub name: Token,
    // value of assignment
    pub value: Expr,
}

/// Grammer for logical expressions.
#[derive(Debug, Clone)]
pub struct ExprLogical {
    // left hand of operation.
    pub left: Expr,
    // operator.
    pub operator: Token,
    // right hand of operation.
    pub right: Expr,
}

/// Grammer for functions call expressions.
#[derive(Debug, Clone)]
pub struct ExprCall {
    // function callee
    pub callee: Expr,
    // paren "(" ")" tokens.
    pub paren: Token,
    // right hand of operation.
    pub arguments: Vec<Expr>,
}

// splax callable.
pub trait SplaxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<TokenLiterals>);
}

#[derive(Debug, Clone)]
pub struct FunctionObject {
    pub declaration: StmtFunc,
}

impl SplaxCallable for FunctionObject {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<TokenLiterals>) {
        // create new environment for function.
        let mut environment = Box::new(Environment::new(Some(interpreter.environment.clone())));

        // define function arguments in new environment.
        for (i, arg) in arguments
            .iter()
            .enumerate()
            .take(self.declaration.params.len())
        {
            environment.define(
                self.declaration.params[i].lexeme.to_string(),
                environment::SplaxDeclarations::Literals(Box::new(arg.clone())),
            )
        }

        if arguments.len() != self.declaration.params.len() {
            App::runtime_error(
                self.declaration.name.line,
                format!(
                    "Expected {} arguments got {}.",
                    self.declaration.params.len(),
                    arguments.len()
                ),
            )
        }

        // interpret function body.
        interpreter.execute_block(&self.declaration.body, environment);
    }
}
