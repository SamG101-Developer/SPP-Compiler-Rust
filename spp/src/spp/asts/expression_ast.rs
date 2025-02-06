use crate::spp::asts::ast::{Ast, ToBinaryExpression};
use crate::spp::asts::binary_expression_ast::BinaryExpressionAst;
use crate::spp::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::spp::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::unary_expression_ast::UnaryExpressionAst;

#[derive(Clone, Debug)]
pub enum ExpressionAst {
    Binary(BinaryExpressionAst),
    Postfix(PostfixExpressionAst),
    Unary(UnaryExpressionAst),
    Primary(PrimaryExpressionAst),
}

impl Ast for ExpressionAst {
    fn get_pos(&self) -> usize {
        match self {
            ExpressionAst::Binary(ast) => ast.get_pos(),
            ExpressionAst::Postfix(ast) => ast.get_pos(),
            ExpressionAst::Unary(ast) => ast.get_pos(),
            ExpressionAst::Primary(ast) => ast.get_pos(),
        }
    }
}

impl ToBinaryExpression for ExpressionAst {
    fn to_binary_expression(pos: usize, lhs: ExpressionAst, op: TokenAst, rhs: Self) -> ExpressionAst {
        ExpressionAst::Binary(BinaryExpressionAst::new(pos, Box::new(lhs), op, Box::new(rhs)))
    }
}
