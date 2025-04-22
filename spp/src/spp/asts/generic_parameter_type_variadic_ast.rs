use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterTypeVariadicAst {
    pos: usize,
    tok_variadic: TokenAst,
    name: TypeAst,
    constraints: Option<GenericParameterConstraintsAst>,
}

impl GenericParameterTypeVariadicAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            name,
            constraints,
        }
    }
}

impl Ast for GenericParameterTypeVariadicAst {
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
