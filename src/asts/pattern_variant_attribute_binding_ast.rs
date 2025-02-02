use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::pattern_variant_ast::PatternVariantNestedForAttributeBindingAst;
use crate::asts::token_ast::TokenAst;

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
