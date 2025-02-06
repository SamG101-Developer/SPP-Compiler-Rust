use crate::spp::asts::ast::Ast;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

impl Ast for LoopElseStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
