use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct UnaryExpressionOperatorAsyncAst {
    pub pos: usize,
    tok_async: TokenAst,
}

impl UnaryExpressionOperatorAsyncAst {
    pub fn new(pos: usize, tok_async: TokenAst) -> Self {
        Self { pos, tok_async }
    }
}

impl Ast for UnaryExpressionOperatorAsyncAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
