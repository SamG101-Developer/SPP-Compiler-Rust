use crate::asts::ast::Ast;
use crate::asts::module_member_ast::ModuleMemberAst;

#[derive(Clone)]
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
