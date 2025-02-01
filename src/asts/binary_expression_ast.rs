use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

pub struct BinaryExpressionAst {
    pub pos: usize,
    pub left: Box<ExpressionAst>,
    pub operator: TokenAst,
    pub right: Box<ExpressionAst>,
}

impl BinaryExpressionAst {
    pub fn new(
        pos: usize,
        left: Box<ExpressionAst>,
        operator: TokenAst,
        right: Box<ExpressionAst>,
    ) -> Self {
        Self {
            pos,
            left,
            operator,
            right,
        }
    }
}
