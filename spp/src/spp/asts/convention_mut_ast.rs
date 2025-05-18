use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ConventionMutAst {
    pub tok_borrow: TokenAst,
    pub tok_mut: TokenAst,
}

impl ConventionMutAst {
    pub fn new(tok_borrow: TokenAst, tok_mut: TokenAst) -> Self {
        Self { tok_borrow, tok_mut }
    }
}

impl Ast for ConventionMutAst {
    fn get_pos(&self) -> usize {
        self.tok_borrow.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_mut.get_final_pos()
    }
}
