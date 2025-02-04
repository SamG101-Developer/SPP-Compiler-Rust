use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::ast::Ast;
use crate::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

pub struct UseStatementAst {
    pub pos: usize,
    pub annotations: Vec<AnnotationAst>,
    pub tok_use: TokenAst,
    pub new_type: TypeAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub tok_assign: TokenAst,
    pub old_type: TypeAst,
}

impl UseStatementAst {
    pub fn new(
        pos: usize,
        annotations: Vec<AnnotationAst>,
        tok_use: TokenAst,
        new_type: TypeAst,
        generic_param_group: Option<GenericParameterGroupAst>,
        tok_assign: TokenAst,
        old_type: TypeAst,
    ) -> Self {
        Self {
            pos,
            annotations,
            tok_use,
            new_type,
            generic_param_group,
            tok_assign,
            old_type,
        }
    }
}

impl Ast for UseStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
