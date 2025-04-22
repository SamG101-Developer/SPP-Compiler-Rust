use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterTypeOptionalAst {
    pos: usize,
    name: TypeAst,
    constraints: Option<GenericParameterConstraintsAst>,
    tok_assign: TokenAst,
    default: TypeAst,
}

impl GenericParameterTypeOptionalAst {
    pub fn new(
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
        tok_assign: TokenAst,
        default: TypeAst,
    ) -> Self {
        Self {
            pos,
            name,
            constraints,
            tok_assign,
            default,
        }
    }
}

impl Ast for GenericParameterTypeOptionalAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.default.get_final_pos()
    }
}
