use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

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
