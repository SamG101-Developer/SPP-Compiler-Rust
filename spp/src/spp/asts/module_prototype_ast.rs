use crate::spp::asts::ast::Ast;
use crate::spp::asts::module_implementation_ast::ModuleImplementationAst;

#[derive(Clone, Debug)]
pub struct ModulePrototypeAst {
    pub body: ModuleImplementationAst,
}

impl ModulePrototypeAst {
    pub fn new(body: ModuleImplementationAst) -> Self {
        Self { body }
    }
}

impl Ast for ModulePrototypeAst {
    fn get_pos(&self) -> usize {
        self.body.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
