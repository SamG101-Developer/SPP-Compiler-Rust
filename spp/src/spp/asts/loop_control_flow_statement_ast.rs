use crate::spp::asts::ast::Ast;
use crate::spp::asts::loop_control_flow_statement_final_part_ast::LoopControlFlowStatementFinalPartAst;
use crate::spp::asts::token_ast::TokenAst;

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

    fn get_final_pos(&self) -> usize {
        if let Some(final_part) = &self.final_part {
            final_part.get_final_pos()
        } else {
            self.tok_exits.last().unwrap().get_final_pos()
        }
    }
}
