use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Debug, Clone)]
pub struct TypeUnaryExpressionOperatorNamespaceAst {
    pos: usize,
    name: IdentifierAst,
    tok_colon: TokenAst,
}

impl TypeUnaryExpressionOperatorNamespaceAst {
    pub fn new(pos: usize, name: IdentifierAst, tok_colon: TokenAst) -> Self {
        Self {
            pos,
            name,
            tok_colon,
        }
    }
}

impl Ast for TypeUnaryExpressionOperatorNamespaceAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_colon.get_final_pos()
    }
}
