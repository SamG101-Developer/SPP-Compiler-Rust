use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;

pub struct LocalVariableSingleIdentifierAliasAst {
    pub pos: usize,
    pub tok_as: TokenAst,
    pub alias: IdentifierAst,
}

impl LocalVariableSingleIdentifierAliasAst {
    pub fn new(pos: usize, tok_as: TokenAst, alias: IdentifierAst) -> Self {
        Self { pos, tok_as, alias }
    }
}
