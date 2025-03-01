use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Debug, Clone)]
pub struct TypePostfixExpressionOperatorIndexedAst {
    pub pos: usize,
    pub tok_colon: TokenAst,
    pub index: TokenAst,
}

impl TypePostfixExpressionOperatorIndexedAst {
    pub fn new(pos: usize, tok_colon: TokenAst, index: TokenAst) -> Self {
        Self {
            pos,
            tok_colon,
            index,
        }
    }
}

impl Ast for TypePostfixExpressionOperatorIndexedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.index.get_final_pos()
    }
}
