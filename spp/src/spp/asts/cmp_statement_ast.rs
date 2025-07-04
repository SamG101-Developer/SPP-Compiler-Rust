use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct CmpStatementAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_cmp: TokenAst,
    pub name: IdentifierAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    pub tok_assign: TokenAst,
    pub value: ExpressionAst,
}

impl CmpStatementAst {
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

impl Ast for CmpStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }

    fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
        self.annotations.iter_mut().try_for_each(|a| a.stage_1_preprocess_asts(context.clone()))?;
        Ok(())
    }
}
