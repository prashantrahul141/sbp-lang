use crate::{
    app::app_main::App,
    ast::ast_tree::{walk_expr, Visitor},
    token::{token_main::TokenLiterals, token_types::TokenType},
};

/// Top level interpreter struct.
pub struct Interpreter;

/// Impl Visitor pattern for Interpreter.
impl Visitor<TokenLiterals> for Interpreter {
    /// Evalute binary expressions.
    /// # Arguments
    /// * `expr` - Binary Expression.
    fn visit_binary_expr(&mut self, expr: &crate::ast::ast_tree::ExprBinary) -> TokenLiterals {
        let left = walk_expr(self, &expr.left);
        let right = walk_expr(self, &expr.right);

        spdlog::debug!("interpreting binary expression: {:?}", expr);

        // matching right literal.
        match right {
            // when right literal is number.
            TokenLiterals::Number(right_value) => match left {
                // when left literal is also number.
                TokenLiterals::Number(left_value) => match expr.operator.token_type {
                    // arithematic operation when both are numbers.
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
                        panic!("This is unreachable, if somehow you managed to trigger this, idk.")
                    }
                },

                // when left is not a number type, while right one is.
                _ => {
                    App::runtime_error(
                        expr.operator.line,
                        format!(
                            "'{}' operator cannot be used with '{}' type.",
                            expr.operator.lexeme, left,
                        ),
                    );
                    panic!();
                }
            },

            // when right literal is a string.
            TokenLiterals::String(right_value) => match left {
                // when left literal is also a string.
                TokenLiterals::String(left_value) => match expr.operator.token_type {
                    // arthematic operators for strings.
                    TokenType::Plus => TokenLiterals::String(format!(
                        "{}{}",
                        &left_value[1..left_value.len() - 1],
                        &right_value[1..right_value.len() - 1],
                    )),

                    // equality operators for strings.
                    TokenType::BangEqual => TokenLiterals::Boolean(left_value != right_value),
                    TokenType::EqualEqual => TokenLiterals::Boolean(left_value == right_value),

                    // any other operator is not for strings.
                    _ => {
                        App::runtime_error(
                            expr.operator.line,
                            format!(
                                "{} operator is not supported between Strings.",
                                expr.operator.lexeme
                            ),
                        );
                        panic!();
                    }
                },

                // when left is not a string type, while right one is.
                _ => {
                    App::runtime_error(
                        expr.operator.line,
                        format!(
                            "'{}' operator cannot be used with type 'String' and '{}' type.",
                            expr.operator.lexeme, left,
                        ),
                    );
                    panic!();
                }
            },

            // when right literal is a boolean.
            TokenLiterals::Boolean(right_value) => match left {
                // when left literal is also a boolean.
                TokenLiterals::Boolean(left_value) => match expr.operator.token_type {
                    // equality operators for booleans.
                    TokenType::BangEqual => TokenLiterals::Boolean(left_value != right_value),
                    TokenType::EqualEqual => TokenLiterals::Boolean(left_value == right_value),
                    _ => {
                        App::runtime_error(
                            expr.operator.line,
                            format!(
                                "{} operator is not supported between Booleans.",
                                expr.operator.lexeme
                            ),
                        );
                        panic!();
                    }
                },

                // when left is not a boolean type, while right one is.
                _ => {
                    App::runtime_error(
                        expr.operator.line,
                        format!(
                            "'{}' operator cannot be used with type 'String' and '{}' type.",
                            expr.operator.lexeme, left,
                        ),
                    );
                    panic!();
                }
            },

            // when right literal is neither string or number.
            _ => {
                App::runtime_error(
                    expr.operator.line,
                    format!("undefined type for '{}' operator.", expr.operator.lexeme),
                );
                panic!();
            }
        }
    }

    /// Evalute group expressions.
    /// # Arguments
    /// * `expr` - Grouping Expression.
    fn visit_grouping_expr(&mut self, expr: &crate::ast::ast_tree::ExprGrouping) -> TokenLiterals {
        spdlog::trace!("interpreting grouping expression: {:?}", expr);
        walk_expr(self, &expr.expression)
    }

    /// Evalute literal expressions.
    /// # Arguments
    /// * `expr` - literal Expression.
    fn visit_literal_expr(&mut self, expr: &crate::ast::ast_tree::ExprLiteral) -> TokenLiterals {
        spdlog::trace!("interpreting literal expression: {:?}", expr);
        expr.value.to_owned()
    }

    /// Evalute unary expressions.
    /// # Arguments
    /// * `expr` - Unary expression.
    fn visit_unary_expr(&mut self, expr: &crate::ast::ast_tree::ExprUnary) -> TokenLiterals {
        spdlog::trace!("interpreting unary expression: {:?}", expr);
        let right = walk_expr(self, &expr.right);

        match expr.operator.token_type {
            TokenType::Minus => match right {
                TokenLiterals::Number(value) => TokenLiterals::Number(-value),
                _ => right,
            },
            TokenType::Bang => self.is_truth(right),
            _ => right,
        }
    }
}
