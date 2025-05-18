use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterTypeRequiredAst {
    name: TypeAst,
    constraints: Option<GenericParameterConstraintsAst>,
}

impl GenericParameterTypeRequiredAst {
    pub fn new(name: TypeAst, constraints: Option<GenericParameterConstraintsAst>) -> Self {
        Self { name, constraints }
    }
}

impl Ast for GenericParameterTypeRequiredAst {
    fn get_pos(&self) -> usize {
        self.name.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.constraints.as_ref().map_or(self.name.get_final_pos(), |c| c.get_final_pos())
    }
}
