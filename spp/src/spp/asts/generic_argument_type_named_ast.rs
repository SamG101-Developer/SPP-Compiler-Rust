use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug, Default)]
pub struct GenericArgumentTypeNamedAst {
    pub pos: usize,
    pub name: TypeAst,
    pub tok_assign: TokenAst,
    pub value: TypeAst,
}

impl GenericArgumentTypeNamedAst {
    pub fn new(pos: usize, name: TypeAst, tok_assign: TokenAst, value: TypeAst) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            value,
        }
    }
}

impl Ast for GenericArgumentTypeNamedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
