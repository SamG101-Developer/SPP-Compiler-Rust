use crate::asts::pattern_variant_ast::PatternVariantNestedForDestructureArrayAst;
use crate::asts::token_ast::TokenAst;

pub struct PatternVariantDestructureArrayAst {
    pub pos: usize,
    pub tok_bracket_l: TokenAst,
    pub elements: Vec<PatternVariantNestedForDestructureArrayAst>,
    pub tok_bracket_r: TokenAst,
}

impl PatternVariantDestructureArrayAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        elements: Vec<PatternVariantNestedForDestructureArrayAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            elements,
            tok_bracket_r,
        }
    }
}
