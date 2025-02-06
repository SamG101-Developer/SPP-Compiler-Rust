use crate::spp::asts::ast::Ast;
use crate::spp::asts::module_implementation_ast::ModuleImplementationAst;

#[derive(Clone, Debug)]
pub struct ModulePrototypeAst {
    pos: usize,
    body: ModuleImplementationAst,
}

impl ModulePrototypeAst {
    pub fn new(pos: usize, body: ModuleImplementationAst) -> Self {
        Self { pos, body }
    }
}

impl Ast for ModulePrototypeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
