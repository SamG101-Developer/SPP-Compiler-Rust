use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct PatternVariantExpressionAst {
    pub expression: ExpressionAst,
}

impl PatternVariantExpressionAst {
    pub fn new(expression: ExpressionAst) -> Self {
        Self { expression }
    }
}

impl Ast for PatternVariantExpressionAst {
    fn get_pos(&self) -> usize {
        self.expression.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.expression.get_final_pos()
    }
}
