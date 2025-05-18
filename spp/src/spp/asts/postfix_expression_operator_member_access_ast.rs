use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionOperatorMemberAccessAst {
    pub tok_access: TokenAst,
    pub field: IdentifierAst,
}

impl PostfixExpressionOperatorMemberAccessAst {
    pub fn new(tok_access: TokenAst, field: IdentifierAst) -> Self {
        Self { tok_access, field }
    }
}

impl Ast for PostfixExpressionOperatorMemberAccessAst {
    fn get_pos(&self) -> usize {
        self.tok_access.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.field.get_final_pos()
    }
}
