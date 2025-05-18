use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;

#[derive(Clone, Debug)]
pub struct LambdaExpressionCaptureItemAst {
    pub convention: ConventionAst,
    pub value: IdentifierAst,
}

impl LambdaExpressionCaptureItemAst {
    pub fn new(convention: ConventionAst, value: IdentifierAst) -> Self {
        Self { convention, value }
    }
}

impl Ast for LambdaExpressionCaptureItemAst {
    fn get_pos(&self) -> usize {
        self.convention.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
