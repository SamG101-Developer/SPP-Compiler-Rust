use crate::asts::pattern_variant_ast::PatternVariantAst;
use crate::asts::token_ast::TokenAst;

pub struct PatternVariantDestructureSkipNArgumentsAst {
    pub pos: usize,
    pub tok_variadic: TokenAst,
    pub binding: Option<PatternVariantAst>,  // SingleIdentifierAst
}

impl PatternVariantDestructureSkipNArgumentsAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        binding: Option<PatternVariantAst>,  // SingleIdentifierAst
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            binding,
        }
    }
}
