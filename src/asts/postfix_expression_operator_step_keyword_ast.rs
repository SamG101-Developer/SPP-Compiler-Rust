use crate::asts::token_ast::TokenAst;

pub struct PostfixExpressionOperatorStepKeywordAst {
    pos: usize,
    tok_access: TokenAst,
    tok_step: TokenAst,
}

impl PostfixExpressionOperatorStepKeywordAst {
    pub fn new(pos: usize, tok_access: TokenAst, tok_step: TokenAst) -> Self {
        Self {
            pos,
            tok_access,
            tok_step,
        }
    }
}
