use std::collections::HashMap;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::loop_condition_ast::LoopConditionAst;
use crate::asts::loop_else_statement_ast::LoopElseStatementAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;
use crate::asts::ast::Ast;

#[derive(Clone)]
pub struct LoopExpressionAst {
    pub pos: usize,
    pub tok_loop: TokenAst,
    pub condition: LoopConditionAst,
    pub body: InnerScopeAst,
    pub else_block: Option<LoopElseStatementAst>,

    _loop_type_info: HashMap<usize, (usize, TypeAst)>,
    _loop_level: usize,
}

impl LoopExpressionAst {
    pub fn new(
        pos: usize,
        tok_loop: TokenAst,
        condition: LoopConditionAst,
        body: InnerScopeAst,
        else_block: Option<LoopElseStatementAst>,
    ) -> Self {
        Self {
            pos,
            tok_loop,
            condition,
            body,
            else_block,
            _loop_type_info: Default::default(),
            _loop_level: Default::default(),
        }
    }
}

impl Ast for LoopExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
