use crate::asts::token_ast::TokenAst;

pub struct ClassAttributeAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub name: IdentifierAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
}

impl ClassAttributeAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        name: IdentifierAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            name,
            tok_colon,
            type_,
        }
    }
}
