use crate::spp::asts::ast::Ast;
use crate::spp::asts::pattern_variant_single_identifier_ast::PatternVariantSingleIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureSkipNArgumentsAst {
    pub tok_variadic: TokenAst,
    pub binding: Option<PatternVariantSingleIdentifierAst>,
}

impl PatternVariantDestructureSkipNArgumentsAst {
    pub fn new(tok_variadic: TokenAst, binding: Option<PatternVariantSingleIdentifierAst>) -> Self {
        Self { tok_variadic, binding }
    }
}

impl Ast for PatternVariantDestructureSkipNArgumentsAst {
    fn get_pos(&self) -> usize {
        self.tok_variadic.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.binding.as_ref().map_or(self.tok_variadic.get_final_pos(), |b| b.get_final_pos())
    }
}
