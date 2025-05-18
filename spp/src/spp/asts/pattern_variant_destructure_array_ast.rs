use crate::spp::asts::ast::Ast;
use crate::spp::asts::pattern_variant_ast::PatternVariantNestedForDestructureArrayAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureArrayAst {
    pub tok_bracket_l: TokenAst,
    pub elements: Vec<PatternVariantNestedForDestructureArrayAst>,
    pub tok_bracket_r: TokenAst,
}

impl PatternVariantDestructureArrayAst {
    pub fn new(tok_bracket_l: TokenAst, elements: Vec<PatternVariantNestedForDestructureArrayAst>, tok_bracket_r: TokenAst) -> Self {
        Self { tok_bracket_l, elements, tok_bracket_r }
    }
}

impl Ast for PatternVariantDestructureArrayAst {
    fn get_pos(&self) -> usize {
        self.tok_bracket_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
