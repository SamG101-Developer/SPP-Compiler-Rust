use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Debug, Clone)]
pub struct TypeUnaryExpressionOperatorNamespaceAst {
    name: IdentifierAst,
    tok_colon: TokenAst,
}

impl TypeUnaryExpressionOperatorNamespaceAst {
    pub fn new(name: IdentifierAst, tok_colon: TokenAst) -> Self {
        Self { name, tok_colon }
    }
}

impl Ast for TypeUnaryExpressionOperatorNamespaceAst {
    fn get_pos(&self) -> usize {
        self.name.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_colon.get_final_pos()
    }
}
