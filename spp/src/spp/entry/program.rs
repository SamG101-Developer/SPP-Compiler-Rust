use crate::spp::asts::ast::Ast;

struct Program { }


impl Ast for Program {
    fn get_pos(&self) -> usize {
        0
    }

    fn get_final_pos(&self) -> usize {
        0
    }
}
