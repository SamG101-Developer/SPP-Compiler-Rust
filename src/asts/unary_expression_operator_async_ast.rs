use crate::asts::token_ast::TokenAst;

pub struct UnaryExpressionOperatorAsyncAst {
    pub pos: usize,
    tok_async: TokenAst,
}

impl UnaryExpressionOperatorAsyncAst {
    pub fn new(pos: usize, tok_async: TokenAst) -> Self {
        Self { pos, tok_async }
    }
}
