use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::local_variable_single_identifier_alias_ast::LocalVariableSingleIdentifierAliasAst;
use crate::asts::token_ast::TokenAst;

pub struct LocalVariableSingleIdentifierAst {
    pub pos: usize,
    pub tok_mut: Option<TokenAst>,
    pub name: IdentifierAst,
    pub alias: Option<LocalVariableSingleIdentifierAliasAst>,
}

impl LocalVariableSingleIdentifierAst {
    pub fn new(
        pos: usize,
        tok_mut: Option<TokenAst>,
        name: IdentifierAst,
        alias: Option<LocalVariableSingleIdentifierAliasAst>,
    ) -> Self {
        Self {
            pos,
            tok_mut,
            name,
            alias,
        }
    }
}
