use crate::asts::expression_ast::ExpressionAst;

pub struct PatternVariantExpressionAst {
    pub pos: usize,
    pub expression: ExpressionAst,
}

impl PatternVariantExpressionAst {
    pub fn new(pos: usize, expression: ExpressionAst) -> Self {
        Self { pos, expression }
    }
}
