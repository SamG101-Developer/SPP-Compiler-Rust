use crate::spp::asts::ast::Ast;
use crate::spp::asts::pattern_variant_ast::PatternVariantNestedForDestructureTupleAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureTupleAst {
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<PatternVariantNestedForDestructureTupleAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl PatternVariantDestructureTupleAst {
    pub fn new(tok_parenthesis_l: TokenAst, elements: Vec<PatternVariantNestedForDestructureTupleAst>, tok_parenthesis_r: TokenAst) -> Self {
        Self { tok_parenthesis_l, elements, tok_parenthesis_r }
    }
}

impl Ast for PatternVariantDestructureTupleAst {
    fn get_pos(&self) -> usize {
        self.tok_parenthesis_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
