use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterTypeVariadicAst {
    tok_variadic: TokenAst,
    name: TypeAst,
    constraints: Option<GenericParameterConstraintsAst>,
}

impl GenericParameterTypeVariadicAst {
    pub fn new(tok_variadic: TokenAst, name: TypeAst, constraints: Option<GenericParameterConstraintsAst>) -> Self {
        Self { tok_variadic, name, constraints }
    }
}

impl Ast for GenericParameterTypeVariadicAst {
    fn get_pos(&self) -> usize {
        self.tok_variadic.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.constraints.as_ref().map_or(self.name.get_final_pos(), |c| c.get_final_pos())
    }
}
