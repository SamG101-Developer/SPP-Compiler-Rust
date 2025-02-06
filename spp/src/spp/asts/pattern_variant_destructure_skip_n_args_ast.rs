use crate::spp::asts::ast::Ast;
use crate::spp::asts::pattern_variant_single_identifier_ast::PatternVariantSingleIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureSkipNArgumentsAst {
    pub pos: usize,
    pub tok_variadic: TokenAst,
    pub binding: Option<PatternVariantSingleIdentifierAst>,
}

impl PatternVariantDestructureSkipNArgumentsAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        binding: Option<PatternVariantSingleIdentifierAst>,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            binding,
        }
    }
}

impl Ast for PatternVariantDestructureSkipNArgumentsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
