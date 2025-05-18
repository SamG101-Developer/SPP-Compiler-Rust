use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantDestructureSkip1ArgumentAst {
    pub tok_underscore: TokenAst,
}

impl PatternVariantDestructureSkip1ArgumentAst {
    pub fn new(tok_underscore: TokenAst) -> Self {
        Self { tok_underscore, }
    }
}

impl Ast for PatternVariantDestructureSkip1ArgumentAst {
    fn get_pos(&self) -> usize {
        self.tok_underscore.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_underscore.get_final_pos()
    }
}
