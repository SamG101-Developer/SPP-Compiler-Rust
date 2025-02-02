use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::token_ast::TokenAst;

pub struct LoopElseStatementAst {
    pos: usize,
    tok_else: TokenAst,
    body: InnerScopeAst,
}

impl LoopElseStatementAst {
    pub fn new(pos: usize, tok_else: TokenAst, body: InnerScopeAst) -> Self {
        Self {
            pos,
            tok_else,
            body,
        }
    }
}
