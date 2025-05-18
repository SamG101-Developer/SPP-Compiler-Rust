use crate::spp::asts::ast::Ast;
use crate::spp::asts::statement_ast::StatementAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct InnerScopeAst {
    tok_brace_l: TokenAst,
    members: Vec<StatementAst>,
    tok_brace_r: TokenAst,
}

impl InnerScopeAst {
    pub fn new(tok_brace_l: TokenAst, members: Vec<StatementAst>, tok_brace_r: TokenAst) -> Self {
        Self { tok_brace_l, members, tok_brace_r }
    }
}

impl Ast for InnerScopeAst {
    fn get_pos(&self) -> usize {
        self.tok_brace_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_brace_r.get_final_pos()
    }
}
