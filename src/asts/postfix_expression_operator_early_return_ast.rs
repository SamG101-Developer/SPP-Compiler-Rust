use crate::asts::token_ast::TokenAst;

pub struct PostfixExpressionOperatorEarlyReturnAst {
    pos: usize,
    tok_qst: TokenAst,
}

impl PostfixExpressionOperatorEarlyReturnAst {
    pub fn new(pos: usize, tok_qst: TokenAst) -> Self {
        Self { pos, tok_qst }
    }
}
