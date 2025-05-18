use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ConventionRefAst {
    pub tok_borrow: TokenAst,
}

impl ConventionRefAst {
    pub fn new(tok_borrow: TokenAst) -> Self {
        Self { tok_borrow }
    }
}

impl Ast for ConventionRefAst {
    fn get_pos(&self) -> usize {
        self.tok_borrow.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_borrow.get_final_pos()
    }
}
