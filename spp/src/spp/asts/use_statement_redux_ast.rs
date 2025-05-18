use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct UseStatementReduxAst {
    pub annotations: Vec<AnnotationAst>,
    pub tok_use: TokenAst,
    pub old_type: TypeAst,
}

impl UseStatementReduxAst {
    pub fn new(annotations: Vec<AnnotationAst>, tok_use: TokenAst, old_type: TypeAst) -> Self {
        Self { annotations, tok_use, old_type }
    }
}

impl Ast for UseStatementReduxAst {
    fn get_pos(&self) -> usize {
        self.annotations.first().map_or(self.tok_use.get_pos(), |a| a.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.old_type.get_final_pos()
    }
}
