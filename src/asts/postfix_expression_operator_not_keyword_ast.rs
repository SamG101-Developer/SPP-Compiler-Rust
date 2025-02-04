use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;

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
}
