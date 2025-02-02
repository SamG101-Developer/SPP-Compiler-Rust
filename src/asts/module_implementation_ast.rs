use crate::asts::module_member_ast::ModuleMemberAst;

pub struct ModuleImplementationAst {
    pos: usize,
    members: Vec<ModuleMemberAst>,
}

impl ModuleImplementationAst {
    pub fn new(pos: usize, members: Vec<ModuleMemberAst>) -> Self {
        Self { pos, members }
    }
}
