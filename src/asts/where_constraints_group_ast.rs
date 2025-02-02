use crate::asts::token_ast::TokenAst;
use crate::asts::where_constraints_ast::WhereConstraintsAst;

pub struct WhereConstraintsGroupAst {
    pub pos: usize,
    pub tok_bracket_l: TokenAst,
    pub constraints: Vec<WhereConstraintsAst>,
    pub tok_bracket_r: TokenAst,
}

impl WhereConstraintsGroupAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        constraints: Vec<WhereConstraintsAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            constraints,
            tok_bracket_r,
        }
    }
}
