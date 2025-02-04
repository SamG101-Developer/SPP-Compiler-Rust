use crate::asts::ast::Ast;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
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
}
