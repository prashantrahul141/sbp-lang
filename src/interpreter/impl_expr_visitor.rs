use super::interpreter_main::Interpreter;
use crate::{
    app::app_main::App,
    ast::expr_ast::{walk_expr, ExprVisitor},
    token::{token_main::TokenLiterals, token_types::TokenType},
};

/// Impl Visitor pattern for Interpreter.
impl ExprVisitor<TokenLiterals> for Interpreter {
    /// Evalute binary expressions.
    /// # Arguments
    /// * `expr` - Binary Expression.
    fn visit_binary_expr(&mut self, expr: &crate::ast::expr_ast::ExprBinary) -> TokenLiterals {
        let left = walk_expr(self, &expr.left);
        let operator = &expr.operator;
        let right = walk_expr(self, &expr.right);

        spdlog::debug!("interpreting binary expression: {:?}", expr);

        // matching left operand.
        match left {
            // if left operand is a number.
            TokenLiterals::Number(left_value) => match right {
                // when both left and right operands are numbers.
                TokenLiterals::Number(right_value) => match operator.token_type {
                    // Operators supported by two number operands.
                    TokenType::Plus => TokenLiterals::Number(left_value + right_value),
                    TokenType::Minus => TokenLiterals::Number(left_value - right_value),
                    TokenType::Star => TokenLiterals::Number(left_value * right_value),
                    TokenType::Slash => TokenLiterals::Number(left_value / right_value),

                    // comparison operator.
                    TokenType::Greater => TokenLiterals::Boolean(left_value > right_value),
                    TokenType::GreaterEqual => TokenLiterals::Boolean(left_value >= right_value),
                    TokenType::Less => TokenLiterals::Boolean(left_value < right_value),
                    TokenType::LessEqual => TokenLiterals::Boolean(left_value <= right_value),

                    // equality operators.
                    TokenType::BangEqual => TokenLiterals::Boolean(left_value != right_value),
                    TokenType::EqualEqual => TokenLiterals::Boolean(left_value == right_value),

                    // any other operators are not for number.
                    _ => {
                        App::runtime_error(
                            operator.line,
                            "unsupported operator for 'Number'".to_string(),
                        );
                        panic!("This is unreachable, if somehow you managed to trigger this, idk.")
                    }
                }, // operator matching for both operands number types.

                // any other type when the left operand is a number will result in a runtime errror.
                _ => {
                    App::runtime_error(
                        operator.line,
                        "unsupported operand type(s): 'Number' with a non 'Number'".to_string(),
                    );
                    panic!();
                }
            }, // left operand matching: Number,

            // when left operand is a string.
            TokenLiterals::String(left_value) => match right {
                // when both left and right operands are strings.
                TokenLiterals::String(right_value) => match operator.token_type {
                    // arthematic operators for strings.
                    TokenType::Plus => TokenLiterals::String(format!(
                        "{}{}",
                        &left_value[1..left_value.len() - 1],
                        &right_value[1..right_value.len() - 1],
                    )),

                    // equality operators for strings.
                    TokenType::BangEqual => TokenLiterals::Boolean(left_value != right_value),
                    TokenType::EqualEqual => TokenLiterals::Boolean(left_value == right_value),

                    // any other operators are not for strings.
                    _ => {
                        App::runtime_error(
                            operator.line,
                            "unsupported operator for 'String'".to_string(),
                        );
                        panic!()
                    }
                }, // operator matching for both operands string types.

                // any other type when the left operand is a string will result in a runtime errror.
                _ => {
                    App::runtime_error(
                        operator.line,
                        "unsupported operand type(s): 'String' with a non 'String'".to_string(),
                    );
                    panic!()
                }
            },

            TokenLiterals::Boolean(left_value) => match right {
                TokenLiterals::Boolean(right_value) => match operator.token_type {
                    // equality operators for booleans.
                    TokenType::BangEqual => TokenLiterals::Boolean(left_value != right_value),
                    TokenType::EqualEqual => TokenLiterals::Boolean(left_value == right_value),

                    // any other operators are not for booleans.
                    _ => {
                        App::runtime_error(
                            operator.line,
                            "unsupported operator for 'Boolean'".to_string(),
                        );
                        panic!()
                    }
                }, // operator matching for both operands boolean types.

                // any other type when the left operand is a boolean will result in a runtime errror.
                _ => {
                    App::runtime_error(
                        operator.line,
                        "unsupported operand type(s): 'Boolean' with a non 'Boolean'".to_string(),
                    );
                    panic!()
                }
            },

            // when right literal is neither number nor string nor boolean.
            _ => {
                App::runtime_error(
                    operator.line,
                    "unsupported operation for this type".to_string(),
                );
                panic!()
            }
        }
    }

    /// Evalute group expressions.
    /// # Arguments
    /// * `expr` - Grouping Expression.
    fn visit_grouping_expr(&mut self, expr: &crate::ast::expr_ast::ExprGrouping) -> TokenLiterals {
        spdlog::trace!("interpreting grouping expression: {:?}", expr);
        walk_expr(self, &expr.expression)
    }

    /// Evalute literal expressions.
    /// # Arguments
    /// * `expr` - literal Expression.
    fn visit_literal_expr(&mut self, expr: &crate::ast::expr_ast::ExprLiteral) -> TokenLiterals {
        spdlog::trace!("interpreting literal expression: {:?}", expr);
        expr.value.to_owned()
    }

    /// Evalute unary expressions.
    /// # Arguments
    /// * `expr` - Unary expression.
    fn visit_unary_expr(&mut self, expr: &crate::ast::expr_ast::ExprUnary) -> TokenLiterals {
        spdlog::trace!("interpreting unary expression: {:?}", expr);
        let right = walk_expr(self, &expr.right);

        match expr.operator.token_type {
            TokenType::Minus => match right {
                TokenLiterals::Number(value) => TokenLiterals::Number(-value),
                _ => right,
            },
            TokenType::Bang => TokenLiterals::Boolean(!self.is_truth(right)),
            _ => right,
        }
    }
}
