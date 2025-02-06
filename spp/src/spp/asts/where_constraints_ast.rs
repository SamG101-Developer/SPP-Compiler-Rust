use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct WhereConstraintsAst {
    pub pos: usize,
    pub types: Vec<TypeAst>,
    pub tok_colon: TokenAst,
    pub constraints: TypeAst,
}

impl WhereConstraintsAst {
    pub fn new(
        pos: usize,
        types: Vec<TypeAst>,
        tok_colon: TokenAst,
        constraints: TypeAst,
    ) -> Self {
        Self {
            pos,
            types,
            tok_colon,
            constraints,
        }
    }
}

impl Ast for WhereConstraintsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.constraints.get_final_pos()
    }
}
