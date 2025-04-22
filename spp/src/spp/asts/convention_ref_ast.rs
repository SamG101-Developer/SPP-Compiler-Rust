use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ConventionRefAst {
    pub pos: usize,
    pub tok_borrow: TokenAst,
}

impl ConventionRefAst {
    pub fn new(pos: usize, tok_borrow: TokenAst) -> Self {
        Self { pos, tok_borrow }
    }
}

impl Ast for ConventionRefAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_borrow.get_final_pos()
    }
}
