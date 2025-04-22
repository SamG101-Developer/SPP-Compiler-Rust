use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum LoopControlFlowStatementFinalPartAst {
    Expression(ExpressionAst),
    Skip(TokenAst),
}
