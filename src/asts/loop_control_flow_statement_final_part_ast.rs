use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
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
}
