use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::unary_expression_operator_ast::UnaryExpressionOperatorAst;

#[derive(Clone)]
pub struct UnaryExpressionAst {
    pub pos: usize,
    pub op: UnaryExpressionOperatorAst,
    pub rhs: Box<ExpressionAst>,
}

impl UnaryExpressionAst {
    pub fn new(pos: usize, op: UnaryExpressionOperatorAst, rhs: Box<ExpressionAst>) -> Self {
        Self { pos, op, rhs }
    }
}

impl Ast for UnaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
