use crate::asts::ast::Ast;
use crate::asts::pattern_variant_single_identifier_ast::PatternVariantSingleIdentifierAst;
use crate::asts::token_ast::TokenAst;

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
