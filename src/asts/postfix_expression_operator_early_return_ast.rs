use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionOperatorEarlyReturnAst {
    pos: usize,
    tok_qst: TokenAst,
}

impl PostfixExpressionOperatorEarlyReturnAst {
    pub fn new(pos: usize, tok_qst: TokenAst) -> Self {
        Self { pos, tok_qst }
    }
}

impl Ast for PostfixExpressionOperatorEarlyReturnAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
