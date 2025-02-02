use crate::asts::ast::Ast;
use crate::asts::binary_expression_ast::BinaryExpressionAst;
use crate::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::asts::unary_expression_ast::UnaryExpressionAst;

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
