use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Debug, Clone)]
pub struct TypePostfixExpressionOperatorNestedAst {
    pub pos: usize,
    pub tok_colon: TokenAst,
    pub name: GenericIdentifierAst,
}

impl TypePostfixExpressionOperatorNestedAst {
    pub fn new(pos: usize, tok_colon: TokenAst, name: GenericIdentifierAst) -> Self {
        Self {
            pos,
            tok_colon,
            name,
        }
    }
}

impl Ast for TypePostfixExpressionOperatorNestedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.name.get_final_pos()
    }
}
