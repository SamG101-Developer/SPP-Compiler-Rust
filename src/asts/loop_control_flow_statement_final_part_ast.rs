use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

pub enum LoopControlFlowStatementFinalPartAst {
    Expression(ExpressionAst),
    Skip(TokenAst),
}
