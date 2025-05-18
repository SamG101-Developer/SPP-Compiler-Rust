use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Debug, Clone)]
pub struct TypePostfixExpressionOperatorOptionalAst {
    pub tok_qst: TokenAst,
}

impl TypePostfixExpressionOperatorOptionalAst {
    pub fn new(tok_qst: TokenAst) -> Self {
        Self { tok_qst }
    }
}

impl Ast for TypePostfixExpressionOperatorOptionalAst {
    fn get_pos(&self) -> usize {
        self.tok_qst.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_qst.get_final_pos()
    }
}
