use crate::spp::asts::ast::Ast;
use crate::spp::asts::statement_ast::StatementAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct InnerScopeAst {
    pos: usize,
    tok_brace_l: TokenAst,
    members: Vec<StatementAst>,
    tok_brace_r: TokenAst,
}

impl InnerScopeAst {
    pub fn new(
        pos: usize,
        tok_brace_l: TokenAst,
        members: Vec<StatementAst>,
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

impl Ast for InnerScopeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
