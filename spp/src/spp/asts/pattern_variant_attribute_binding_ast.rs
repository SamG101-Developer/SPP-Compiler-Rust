use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::pattern_variant_ast::PatternVariantNestedForAttributeBindingAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantAttributeBindingAst {
    pub pos: usize,
    pub name: IdentifierAst,
    pub tok_assign: TokenAst,
    pub value: PatternVariantNestedForAttributeBindingAst,
}

impl PatternVariantAttributeBindingAst {
    pub fn new(
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        value: PatternVariantNestedForAttributeBindingAst,
    ) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            value,
        }
    }
}

impl Ast for PatternVariantAttributeBindingAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
