use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterTypeRequiredAst {
    pos: usize,
    name: TypeAst,
    constraints: Option<GenericParameterConstraintsAst>,
}

impl GenericParameterTypeRequiredAst {
    pub fn new(
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    ) -> Self {
        Self {
            pos,
            name,
            constraints,
        }
    }
}

impl Ast for GenericParameterTypeRequiredAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        if let Some(constraints) = &self.constraints {
            constraints.get_final_pos()
        } else {
            self.name.get_final_pos()
        }
    }
}
