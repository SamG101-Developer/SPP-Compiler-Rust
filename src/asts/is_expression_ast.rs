use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::pattern_variant_ast::PatternVariantAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct IsExpressionAst {
    pub pos: usize,
    pub left: Box<ExpressionAst>,
    pub operator: TokenAst,
    pub right: Box<PatternVariantAst>,
}

impl IsExpressionAst {
    pub fn new(
        pos: usize,
        left: Box<ExpressionAst>,
        operator: TokenAst,
        right: Box<PatternVariantAst>,
    ) -> Self {
        Self {
            pos,
            left,
            operator,
            right,
        }
    }
}

impl Ast for IsExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
