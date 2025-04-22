use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ConventionMutAst {
    pub pos: usize,
    pub tok_borrow: TokenAst,
    pub tok_mut: TokenAst,
}

impl ConventionMutAst {
    pub fn new(pos: usize, tok_borrow: TokenAst, tok_mut: TokenAst) -> Self {
        Self {
            pos,
            tok_borrow,
            tok_mut,
        }
    }
}

impl Ast for ConventionMutAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_mut.get_final_pos()
    }
}
