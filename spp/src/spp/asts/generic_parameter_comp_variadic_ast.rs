use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterCompVariadicAst {
    pos: usize,
    tok_cmp: TokenAst,
    tok_variadic: TokenAst,
    name: TypeAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl GenericParameterCompVariadicAst {
    pub fn new(
        pos: usize,
        tok_cmp: TokenAst,
        tok_variadic: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_cmp,
            tok_variadic,
            name,
            tok_colon,
            type_,
        }
    }
}

impl Ast for GenericParameterCompVariadicAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
