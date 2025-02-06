use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub enum LoopControlFlowStatementFinalPartAst {
    Expression(ExpressionAst),
    Skip(TokenAst),
}

impl Ast for LoopControlFlowStatementFinalPartAst {
    fn get_pos(&self) -> usize {
        match self {
            LoopControlFlowStatementFinalPartAst::Expression(expr) => expr.get_pos(),
            LoopControlFlowStatementFinalPartAst::Skip(tok) => tok.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LoopControlFlowStatementFinalPartAst::Expression(expr) => expr.get_final_pos(),
            LoopControlFlowStatementFinalPartAst::Skip(tok) => tok.get_final_pos(),
        }
    }
}
