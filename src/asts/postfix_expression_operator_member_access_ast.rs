use crate::asts::ast::Ast;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionOperatorMemberAccessAst {
    pub pos: usize,
    pub tok_access: TokenAst,
    pub field: IdentifierAst,
}

impl PostfixExpressionOperatorMemberAccessAst {
    pub fn new(pos: usize, tok_access: TokenAst, field: IdentifierAst) -> Self {
        Self {
            pos,
            tok_access,
            field,
        }
    }
}

impl Ast for PostfixExpressionOperatorMemberAccessAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
