use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;
use crate::asts::where_constraints_group_ast::WhereConstraintsGroupAst;

#[derive(Clone, Debug)]
pub struct WhereBlockAst {
    pub pos: usize,
    pub tok_where: TokenAst,
    pub constraints: WhereConstraintsGroupAst,
}

impl WhereBlockAst {
    pub fn new(pos: usize, tok_where: TokenAst, constraints: WhereConstraintsGroupAst) -> Self {
        Self {
            pos,
            tok_where,
            constraints,
        }
    }
}

impl Ast for WhereBlockAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
