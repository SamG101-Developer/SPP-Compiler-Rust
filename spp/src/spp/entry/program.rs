use crate::spp::asts::ast::Ast;

pub struct Program {}

impl Program {
    pub fn new() -> Self {
        Self {}
    }
}

impl Ast for Program {
    fn get_pos(&self) -> usize {
        0
    }

    fn get_final_pos(&self) -> usize {
        0
    }
}
