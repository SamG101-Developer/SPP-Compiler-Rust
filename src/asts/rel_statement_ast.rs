use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

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
