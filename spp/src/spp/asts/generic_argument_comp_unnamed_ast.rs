use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct GenericArgumentCompUnnamedAst {
    value: ExpressionAst,
}

impl GenericArgumentCompUnnamedAst {
    pub fn new(value: ExpressionAst) -> Self {
        Self { value }
    }
}

impl Ast for GenericArgumentCompUnnamedAst {
    fn get_pos(&self) -> usize {
        self.value.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
