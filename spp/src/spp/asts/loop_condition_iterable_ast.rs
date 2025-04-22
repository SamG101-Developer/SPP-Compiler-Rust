use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LoopConditionIterableAst {
    pos: usize,
    variable: LocalVariableAst,
    tok_in: TokenAst,
    iterable: Box<ExpressionAst>,
}

impl LoopConditionIterableAst {
    pub fn new(
        pos: usize,
        variable: LocalVariableAst,
        tok_in: TokenAst,
        iterable: Box<ExpressionAst>,
    ) -> Self {
        Self {
            pos,
            variable,
            tok_in,
            iterable,
        }
    }
}

impl Ast for LoopConditionIterableAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.iterable.get_final_pos()
    }
}
