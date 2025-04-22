use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LiteralStringAst {
    pos: usize,
    value: TokenAst,
}

impl LiteralStringAst {
    pub fn new(pos: usize, value: TokenAst) -> Self {
        Self { pos, value }
    }
}

impl Ast for LiteralStringAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
