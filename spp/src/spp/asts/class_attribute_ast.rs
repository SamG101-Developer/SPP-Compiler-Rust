use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct ClassAttributeAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub name: IdentifierAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    pub default: Option<ExpressionAst>,
}

impl ClassAttributeAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        name: IdentifierAst,
        tok_colon: TokenAst,
        type_: TypeAst,
        default: Option<ExpressionAst>,
    ) -> Self {
        Self {
            pos,
            annotations,
            name,
            tok_colon,
            type_,
            default,
        }
    }
}

impl Ast for ClassAttributeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
