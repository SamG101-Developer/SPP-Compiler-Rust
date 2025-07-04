use crate::spp::asts::ast::Ast;
use crate::spp::asts::module_implementation_ast::ModuleImplementationAst;

#[derive(Clone, Debug)]
pub struct ModulePrototypeAst {
    pub pos: usize,
    pub body: ModuleImplementationAst,
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

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
