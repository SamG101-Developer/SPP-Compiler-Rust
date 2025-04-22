use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use enum_dispatch::enum_dispatch;
use crate::spp::asts::assignment_statement_ast::AssignmentStatementAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::let_statement_ast::LetStatementAst;
use crate::spp::asts::loop_control_flow_statement_ast::LoopControlFlowStatementAst;
use crate::spp::asts::ret_statement_ast::RetStatementAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum StatementAst {
    Assignment(AssignmentStatementAst),
    Expression(ExpressionAst),
    Let(LetStatementAst),
    LoopControlFlow(LoopControlFlowStatementAst),
    Ret(RetStatementAst),
    Use(UseStatementAst),
}
