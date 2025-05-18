use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterConstraintsAst {
    pub tok_colon: TokenAst,
    pub constraint: TypeAst,
}

impl GenericParameterConstraintsAst {
    pub fn new(tok_colon: TokenAst, constraint: TypeAst) -> Self {
        Self { tok_colon, constraint }
    }
}

impl Ast for GenericParameterConstraintsAst {
    fn get_pos(&self) -> usize {
        self.tok_colon.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.constraint.get_final_pos()
    }
}
