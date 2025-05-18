use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LiteralBooleanAst {
    pub value: TokenAst,
}

impl LiteralBooleanAst {
    pub fn new(value: TokenAst) -> Self {
        Self { value }
    }
}

impl Ast for LiteralBooleanAst {
    fn get_pos(&self) -> usize {
        self.value.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
