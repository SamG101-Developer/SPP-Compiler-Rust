use crate::asts::expression_ast::ExpressionAst;

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
