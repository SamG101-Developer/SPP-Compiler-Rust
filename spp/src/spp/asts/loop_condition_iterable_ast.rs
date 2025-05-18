use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LoopConditionIterableAst {
    variable: LocalVariableAst,
    tok_in: TokenAst,
    iterable: Box<ExpressionAst>,
}

impl LoopConditionIterableAst {
    pub fn new(variable: LocalVariableAst, tok_in: TokenAst, iterable: Box<ExpressionAst>) -> Self {
        Self { variable, tok_in, iterable }
    }
}

impl Ast for LoopConditionIterableAst {
    fn get_pos(&self) -> usize {
        self.variable.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.iterable.get_final_pos()
    }
}
