use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

impl Ast for PostfixExpressionOperatorStepKeywordAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
