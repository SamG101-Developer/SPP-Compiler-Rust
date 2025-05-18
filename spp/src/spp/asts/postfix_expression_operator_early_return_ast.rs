use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionOperatorEarlyReturnAst {
    tok_qst: TokenAst,
}

impl PostfixExpressionOperatorEarlyReturnAst {
    pub fn new(tok_qst: TokenAst) -> Self {
        Self { tok_qst }
    }
}

impl Ast for PostfixExpressionOperatorEarlyReturnAst {
    fn get_pos(&self) -> usize {
        self.tok_qst.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_qst.get_final_pos()
    }
}
