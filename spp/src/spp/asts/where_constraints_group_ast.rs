use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::where_constraints_ast::WhereConstraintsAst;

#[derive(Clone, Debug)]
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

impl Ast for WhereConstraintsGroupAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
