use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub enum LoopConditionAst {
    Boolean {
        pos: usize,
        condition: Box<ExpressionAst>,
    },
    Iterable {
        pos: usize,
        variable: LocalVariableAst,
        tok_in: TokenAst,
        iterable: Box<ExpressionAst>,
    },
}

impl LoopConditionAst {
    pub fn new_boolean(pos: usize, condition: Box<ExpressionAst>) -> Self {
        Self::Boolean { pos, condition }
    }

    pub fn new_iterable(
        pos: usize,
        variable: LocalVariableAst,
        tok_in: TokenAst,
        iterable: Box<ExpressionAst>,
    ) -> Self {
        Self::Iterable {
            pos,
            variable,
            tok_in,
            iterable,
        }
    }
}

impl Ast for LoopConditionAst {
    fn get_pos(&self) -> usize {
        match self {
            LoopConditionAst::Boolean { pos, .. } => *pos,
            LoopConditionAst::Iterable { pos, .. } => *pos,
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LoopConditionAst::Boolean { condition, .. } => condition.get_final_pos(),
            LoopConditionAst::Iterable { iterable, .. } => iterable.get_final_pos(),
        }
    }
}
