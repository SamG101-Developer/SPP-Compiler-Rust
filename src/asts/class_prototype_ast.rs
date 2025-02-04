use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::ast::Ast;
use crate::asts::class_implementation_ast::ClassImplementationAst;
use crate::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;
use crate::asts::where_block_ast::WhereBlockAst;

#[derive(Clone)]
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
}
