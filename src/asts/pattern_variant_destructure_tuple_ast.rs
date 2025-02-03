use crate::asts::ast::Ast;
use crate::asts::pattern_variant_ast::PatternVariantNestedForDestructureTupleAst;
use crate::asts::token_ast::TokenAst;

pub struct PatternVariantDestructureTupleAst {
    pub pos: usize,
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<PatternVariantNestedForDestructureTupleAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl PatternVariantDestructureTupleAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        elements: Vec<PatternVariantNestedForDestructureTupleAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for PatternVariantDestructureTupleAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
