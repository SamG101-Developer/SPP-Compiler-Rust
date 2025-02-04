use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::local_variable_ast::LocalVariableAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
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
}
