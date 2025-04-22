use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericArgumentTypeUnnamedAst {
    pos: usize,
    type_: TypeAst,
}

impl GenericArgumentTypeUnnamedAst {
    pub fn new(pos: usize, type_: TypeAst) -> Self {
        Self { pos, type_ }
    }
}

impl Ast for GenericArgumentTypeUnnamedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
