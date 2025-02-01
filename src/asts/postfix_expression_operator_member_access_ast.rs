use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;

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
