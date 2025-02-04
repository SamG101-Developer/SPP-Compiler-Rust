use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

pub trait Ast {
    fn get_pos(&self) -> usize;
}

pub trait ToBinaryExpression {
    fn to_binary_expression(pos: usize, lhs: ExpressionAst, op: TokenAst, rhs: Self) -> ExpressionAst;
}
