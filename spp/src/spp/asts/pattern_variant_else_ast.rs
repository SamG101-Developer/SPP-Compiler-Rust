use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantElseAst {
    pub tok_else: TokenAst,
}

impl PatternVariantElseAst {
    pub fn new(tok_else: TokenAst) -> Self {
        Self { tok_else }
    }
}

impl Ast for PatternVariantElseAst {
    fn get_pos(&self) -> usize {
        self.tok_else.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_else.get_final_pos()
    }
}
