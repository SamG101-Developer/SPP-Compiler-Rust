use std::collections::HashMap;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::loop_condition_ast::LoopConditionAst;
use crate::spp::asts::loop_else_statement_ast::LoopElseStatementAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::ast::Ast;

#[derive(Clone, Debug)]
pub struct LoopExpressionAst {
    pub tok_loop: TokenAst,
    pub condition: LoopConditionAst,
    pub body: InnerScopeAst,
    pub else_block: Option<LoopElseStatementAst>,

    _loop_type_info: HashMap<usize, (usize, TypeAst)>,
    _loop_level: usize,
}

impl LoopExpressionAst {
    pub fn new(tok_loop: TokenAst, condition: LoopConditionAst, body: InnerScopeAst, else_block: Option<LoopElseStatementAst>) -> Self {
        Self { tok_loop, condition, body, else_block, _loop_type_info: Default::default(), _loop_level: Default::default() }
    }
}

impl Ast for LoopExpressionAst {
    fn get_pos(&self) -> usize {
        self.tok_loop.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.else_block.as_ref().map_or(self.body.get_final_pos(), |t| t.get_final_pos())
    }
}
