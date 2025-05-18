use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LocalVariableSingleIdentifierAliasAst {
    pub tok_as: TokenAst,
    pub alias: IdentifierAst,
}

impl LocalVariableSingleIdentifierAliasAst {
    pub fn new(tok_as: TokenAst, alias: IdentifierAst) -> Self {
        Self { tok_as, alias }
    }
}

impl Ast for LocalVariableSingleIdentifierAliasAst {
    fn get_pos(&self) -> usize {
        self.tok_as.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.alias.get_final_pos()
    }
}
