use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::local_variable_single_identifier_alias_ast::LocalVariableSingleIdentifierAliasAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantSingleIdentifierAst {
    pub tok_mut: Option<TokenAst>,
    pub name: IdentifierAst,
    pub alias: Option<LocalVariableSingleIdentifierAliasAst>,
}

impl PatternVariantSingleIdentifierAst {
    pub fn new(tok_mut: Option<TokenAst>, name: IdentifierAst, alias: Option<LocalVariableSingleIdentifierAliasAst>) -> Self {
        Self { tok_mut, name, alias }
    }
}

impl Ast for PatternVariantSingleIdentifierAst {
    fn get_pos(&self) -> usize {
        self.tok_mut.as_ref().map_or(self.name.get_pos(), |t| t.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.alias.as_ref().map_or(self.name.get_final_pos(), |a| a.get_final_pos())
    }
}
