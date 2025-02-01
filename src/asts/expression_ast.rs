use crate::asts::binary_expression_ast::BinaryExpressionAst;

pub enum ExpressionAst {
    Binary(BinaryExpressionAst),
    Postfix(PostfixExpressionAst),
    Unary(UnaryExpressionAst),
    Primary(PrimaryExpressionAst),
}
