use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct GenericArgumentCompUnnamedAst {
    pos: usize,
    value: ExpressionAst,
}

impl GenericArgumentCompUnnamedAst {
    pub fn new(pos: usize, value: ExpressionAst) -> Self {
        Self { pos, value }
    }
}

impl Ast for GenericArgumentCompUnnamedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
