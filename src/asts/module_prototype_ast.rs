use crate::asts::ast::Ast;
use crate::asts::module_implementation_ast::ModuleImplementationAst;

#[derive(Clone)]
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
