use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct RelStatementAst {
    pub pos: usize,
    pub tok_rel: TokenAst,
    pub expressions: Vec<ExpressionAst>,
}

impl RelStatementAst {
    pub fn new(pos: usize, tok_rel: TokenAst, expressions: Vec<ExpressionAst>) -> Self {
        Self {
            pos,
            tok_rel,
            expressions,
        }
    }
}

impl Ast for RelStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
