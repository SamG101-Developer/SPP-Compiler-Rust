use crate::spp::asts::ast::Ast;
use crate::spp::asts::module_member_ast::ModuleMemberAst;

#[derive(Clone, Debug)]
pub struct ModuleImplementationAst {
    pos: usize,
    members: Vec<ModuleMemberAst>,
}

impl ModuleImplementationAst {
    pub fn new(pos: usize, members: Vec<ModuleMemberAst>) -> Self {
        Self { pos, members }
    }
}

impl Ast for ModuleImplementationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
