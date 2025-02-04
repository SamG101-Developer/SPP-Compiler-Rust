use crate::asts::ast::Ast;
use crate::asts::pattern_variant_ast::PatternVariantNestedForDestructureObjectAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureObjectAst {
    pub pos: usize,
    pub type_: TypeAst,
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<PatternVariantNestedForDestructureObjectAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl PatternVariantDestructureObjectAst {
    pub fn new(
        pos: usize,
        type_: TypeAst,
        tok_parenthesis_l: TokenAst,
        elements: Vec<PatternVariantNestedForDestructureObjectAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            type_,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for PatternVariantDestructureObjectAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
