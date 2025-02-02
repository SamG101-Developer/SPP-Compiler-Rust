use crate::asts::assignment_statement_ast::AssignmentStatementAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::let_statement_ast::LetStatementAst;
use crate::asts::loop_control_flow_statement_ast::LoopControlFlowStatementAst;
use crate::asts::pin_statement_ast::PinStatementAst;
use crate::asts::rel_statement_ast::RelStatementAst;
use crate::asts::ret_statement_ast::RetStatementAst;
use crate::asts::use_statement_ast::UseStatementAst;

pub enum StatementAst {
    Assignment(AssignmentStatementAst),
    Expression(ExpressionAst),
    Let(LetStatementAst),
    LoopControlFlow(LoopControlFlowStatementAst),
    Pin(PinStatementAst),
    Rel(RelStatementAst),
    Ret(RetStatementAst),
    Use(UseStatementAst),
}
