use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::class_implementation_ast::ClassImplementationAst;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;

#[derive(Clone, Debug, Default)]
pub struct ClassPrototypeAst {
    pub annotations: Vec<AnnotationAst>,
    pub tok_cls: TokenAst,
    pub name: TypeAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub where_block: Option<WhereBlockAst>,
    pub body: ClassImplementationAst,
    pub is_alias: bool,

    _ctx: Option<PreProcessingContext>
}

impl ClassPrototypeAst {
    pub fn new(
        annotations: Vec<AnnotationAst>,
        tok_cls: TokenAst,
        name: TypeAst,
        generic_param_group: Option<GenericParameterGroupAst>,
        where_block: Option<WhereBlockAst>,
        body: ClassImplementationAst,
    ) -> Self {
        Self { annotations, tok_cls, name, generic_param_group, where_block, body, is_alias: false, _ctx: None, }
    }
}

impl Ast for ClassPrototypeAst {
    fn get_pos(&self) -> usize {
        self.annotations.first().map_or(self.tok_cls.get_pos(), |a| a.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        if let Some(generic_param_group) = &self.generic_param_group {
            generic_param_group.get_final_pos()
        } else {
            self.name.get_final_pos()
        }
    }

    // fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
    //     self._ctx = Some(context.clone());
    //     self.annotations.iter_mut().try_for_each(|a| a.stage_1_preprocess_asts(context.clone()))?;
    //     self.body.stage_1_preprocess_asts(context)?;
    //     Ok(())
    // }
}
