use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_implementation_ast::ClassImplementationAst;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;

#[derive(Clone, Debug)]
pub struct ClassPrototypeAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_cls: TokenAst,
    pub name: TypeAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub where_block: Option<WhereBlockAst>,
    pub body: ClassImplementationAst,
    pub is_alias: bool,
}

impl ClassPrototypeAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        tok_cls: TokenAst,
        name: TypeAst,
        generic_param_group: Option<GenericParameterGroupAst>,
        where_block: Option<WhereBlockAst>,
        body: ClassImplementationAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            tok_cls,
            name,
            generic_param_group,
            where_block,
            body,
            is_alias: false,
        }
    }
}

impl Ast for ClassPrototypeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
