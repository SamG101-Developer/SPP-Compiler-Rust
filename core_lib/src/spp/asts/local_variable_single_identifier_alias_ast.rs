use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

impl Ast for LocalVariableSingleIdentifierAliasAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
