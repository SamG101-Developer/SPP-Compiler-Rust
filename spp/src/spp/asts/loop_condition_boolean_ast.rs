use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;

#[derive(Clone, Debug)]
pub struct LoopConditionBooleanAst {
    condition: Box<ExpressionAst>,
}

impl LoopConditionBooleanAst {
    pub fn new(condition: Box<ExpressionAst>) -> Self {
        Self { condition }
    }
}

impl Ast for LoopConditionBooleanAst {
    fn get_pos(&self) -> usize {
        self.condition.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.condition.get_final_pos()
    }
}
