use crate::asts::sup_member_ast::SupMemberAst;
use crate::asts::token_ast::TokenAst;

pub struct SupImplementationAst {
    pub pos: usize,
    pub tok_brace_l: TokenAst,
    pub members: Vec<SupMemberAst>,
    pub tok_brace_r: TokenAst,
}

impl SupImplementationAst {
    pub fn new(
        pos: usize,
        tok_brace_l: TokenAst,
        members: Vec<SupMemberAst>,
        tok_brace_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_brace_l,
            members,
            tok_brace_r,
        }
    }
}
