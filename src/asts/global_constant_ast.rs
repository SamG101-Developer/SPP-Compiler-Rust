use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone)]
pub struct GlobalConstantAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_cmp: TokenAst,
    pub name: IdentifierAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    pub tok_assign: TokenAst,
    pub value: ExpressionAst,
}

impl GlobalConstantAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        tok_cmp: TokenAst,
        name: IdentifierAst,
        tok_colon: TokenAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            tok_cmp,
            name,
            tok_colon,
            type_,
            tok_assign,
            value,
        }
    }
}

impl Ast for GlobalConstantAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
