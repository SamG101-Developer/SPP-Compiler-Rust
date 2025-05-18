use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct AnnotationAst {
    pub tok_at: TokenAst,
    pub name: IdentifierAst,
}

impl AnnotationAst {
    pub fn new(tok_at: TokenAst, name: IdentifierAst) -> Self {
        Self { tok_at, name }
    }
}

impl Ast for AnnotationAst {
    fn get_pos(&self) -> usize {
        self.tok_at.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.name.get_final_pos()
    }
}
