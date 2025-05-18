use crate::spp::asts::ast::Ast;
use crate::spp::asts::loop_control_flow_statement_final_part_ast::LoopControlFlowStatementFinalPartAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LoopControlFlowStatementAst {
    pub tok_exits: Vec<TokenAst>,
    pub final_part: Option<LoopControlFlowStatementFinalPartAst>,
}

impl LoopControlFlowStatementAst {
    pub fn new(tok_exits: Vec<TokenAst>, final_part: Option<LoopControlFlowStatementFinalPartAst>) -> Self {
        Self {tok_exits, final_part }
    }
}

impl Ast for LoopControlFlowStatementAst {
    fn get_pos(&self) -> usize {
        self.tok_exits.first().map_or(self.final_part.as_ref().unwrap().get_pos(), |t| t.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.final_part.as_ref().map_or(self.tok_exits.last().unwrap().get_final_pos(), |t| t.get_final_pos())
    }
}
