use crate::asts::token_ast::TokenAst;

pub struct WhereConstraintsAst {
    pub pos: usize,
    pub types: Vec<TypeAst>,
    pub tok_colon: TokenAst,
    pub constraints: Vec<TypeAst>,
}

impl WhereConstraintsAst {
    pub fn new(
        pos: usize,
        types: Vec<TypeAst>,
        tok_colon: TokenAst,
        constraints: Vec<TypeAst>,
    ) -> Self {
        Self {
            pos,
            types,
            tok_colon,
            constraints,
        }
    }
}
