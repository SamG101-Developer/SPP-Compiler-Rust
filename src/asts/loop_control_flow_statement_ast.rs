use crate::asts::ast::Ast;
use crate::asts::loop_control_flow_statement_final_part_ast::LoopControlFlowStatementFinalPartAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LoopControlFlowStatementAst {
    pub pos: usize,
    pub tok_exits: Vec<TokenAst>,
    pub final_part: Option<LoopControlFlowStatementFinalPartAst>,
}

impl LoopControlFlowStatementAst {
    pub fn new(
        pos: usize,
        tok_exits: Vec<TokenAst>,
        final_part: Option<LoopControlFlowStatementFinalPartAst>,
    ) -> Self {
        Self {
            pos,
            tok_exits,
            final_part,
        }
    }
}

impl Ast for LoopControlFlowStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
