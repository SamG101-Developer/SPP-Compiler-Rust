use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct UseStatementAliasAst {
    pub annotations: Vec<AnnotationAst>,
    pub tok_use: TokenAst,
    pub new_type: TypeAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub tok_assign: TokenAst,
    pub old_type: TypeAst,

    _ctx: Option<PreProcessingContext>
}

impl UseStatementAliasAst {
    pub fn new(annotations: Vec<AnnotationAst>, tok_use: TokenAst, new_type: TypeAst, generic_param_group: Option<GenericParameterGroupAst>, tok_assign: TokenAst, old_type: TypeAst) -> Self {
        Self { annotations, tok_use, new_type, generic_param_group, tok_assign, old_type, _ctx: None }
    }
}

impl Ast for UseStatementAliasAst {
    fn get_pos(&self) -> usize {
        self.annotations.first().map_or(self.tok_use.get_pos(), |a| a.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.old_type.get_final_pos()
    }

    // fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
    //     self._ctx = Some(context.clone());
    //     self.annotations.iter_mut().try_for_each(|a| a.stage_1_preprocess_asts(context.clone()))?;
    //     Ok(())
    // }
}
