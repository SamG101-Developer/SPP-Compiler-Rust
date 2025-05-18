use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::pattern_variant_ast::PatternVariantAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct IsExpressionAst {
    pub left: Box<ExpressionAst>,
    pub operator: TokenAst,
    pub right: Box<PatternVariantAst>,
}

impl IsExpressionAst {
    pub fn new(left: Box<ExpressionAst>, operator: TokenAst, right: Box<PatternVariantAst>) -> Self {
        Self { left, operator, right }
    }
}

impl Ast for IsExpressionAst {
    fn get_pos(&self) -> usize {
        self.left.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.right.get_final_pos()
    }
}
