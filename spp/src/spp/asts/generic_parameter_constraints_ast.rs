use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterConstraintsAst {
    pub pos: usize,
    pub tok_colon: TokenAst,
    pub constraint: TypeAst,
}

impl GenericParameterConstraintsAst {
    pub fn new(pos: usize, tok_colon: TokenAst, constraint: TypeAst) -> Self {
        Self {
            pos,
            tok_colon,
            constraint,
        }
    }
}

impl Ast for GenericParameterConstraintsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.constraint.get_final_pos()
    }
}
