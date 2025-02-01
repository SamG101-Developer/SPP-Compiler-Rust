use crate::asts::module_implementation_ast::ModuleImplementationAst;

pub struct ModulePrototypeAst {
    pos: usize,
    body: ModuleImplementationAst,
}

impl ModulePrototypeAst {
    pub fn new(pos: usize, body: ModuleImplementationAst) -> Self {
        Self { pos, body }
    }
}
