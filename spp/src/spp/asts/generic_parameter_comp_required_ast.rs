use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterCompRequiredAst {
    pos: usize,
    tok_comp: TokenAst,
    name: TypeAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl GenericParameterCompRequiredAst {
    pub fn new(
        pos: usize,
        tok_comp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_comp,
            name,
            tok_colon,
            type_,
        }
    }
}

impl Ast for GenericParameterCompRequiredAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
