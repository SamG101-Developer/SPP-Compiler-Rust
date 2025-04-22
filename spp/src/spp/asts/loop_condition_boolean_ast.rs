use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct LoopConditionBooleanAst {
    pos: usize,
    condition: Box<ExpressionAst>,
}

impl LoopConditionBooleanAst {
    pub fn new(pos: usize, condition: Box<ExpressionAst>) -> Self {
        Self { pos, condition }
    }
}

impl Ast for LoopConditionBooleanAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.condition.get_final_pos()
    }
}
