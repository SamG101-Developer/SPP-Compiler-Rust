use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionOperatorNotKeywordAst {
    pos: usize,
    tok_access: TokenAst,
    tok_not: TokenAst,
}

impl PostfixExpressionOperatorNotKeywordAst {
    pub fn new(pos: usize, tok_access: TokenAst, tok_not: TokenAst) -> Self {
        Self {
            pos,
            tok_access,
            tok_not,
        }
    }
}

impl Ast for PostfixExpressionOperatorNotKeywordAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_not.get_final_pos()
    }
}
