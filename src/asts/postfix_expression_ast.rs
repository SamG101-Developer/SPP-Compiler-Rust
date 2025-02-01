use crate::asts::expression_ast::ExpressionAst;
use crate::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;

pub struct PostfixExpressionAst {
    pub pos: usize,
    pub lhs: Box<ExpressionAst>,
    pub op: Vec<PostfixExpressionOperatorAst>,
}

impl PostfixExpressionAst {
    pub fn new(pos: usize, lhs: Box<ExpressionAst>, op: Vec<PostfixExpressionOperatorAst>) -> Self {
        Self { pos, lhs, op }
    }
}
