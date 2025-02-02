use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::ast::Ast;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

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

impl Ast for ClassAttributeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
