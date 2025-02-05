use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct PatternVariantExpressionAst {
    pub pos: usize,
    pub expression: ExpressionAst,
}

impl PatternVariantExpressionAst {
    pub fn new(pos: usize, expression: ExpressionAst) -> Self {
        Self { pos, expression }
    }
}

impl Ast for PatternVariantExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
