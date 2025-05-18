use crate::spp::asts::ast::Ast;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LoopElseStatementAst {
    tok_else: TokenAst,
    body: InnerScopeAst,
}

impl LoopElseStatementAst {
    pub fn new(tok_else: TokenAst, body: InnerScopeAst) -> Self {
        Self { tok_else, body }
    }
}

impl Ast for LoopElseStatementAst {
    fn get_pos(&self) -> usize {
        self.tok_else.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
