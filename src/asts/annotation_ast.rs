use crate::asts::token_ast::TokenAst;

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
