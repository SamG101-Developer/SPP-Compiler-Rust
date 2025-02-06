use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct AnnotationAst {
    pub pos: usize,
    pub tok_at: TokenAst,
    pub name: IdentifierAst,
}

impl AnnotationAst {
    pub fn new(pos: usize, tok_at: TokenAst, name: IdentifierAst) -> Self {
        Self { pos, tok_at, name }
    }
}

impl Ast for AnnotationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.name.get_final_pos()
    }
}
