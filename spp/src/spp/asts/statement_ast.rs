use crate::spp::asts::assignment_statement_ast::AssignmentStatementAst;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::let_statement_ast::LetStatementAst;
use crate::spp::asts::loop_control_flow_statement_ast::LoopControlFlowStatementAst;
use crate::spp::asts::pin_statement_ast::PinStatementAst;
use crate::spp::asts::rel_statement_ast::RelStatementAst;
use crate::spp::asts::ret_statement_ast::RetStatementAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;

#[derive(Clone, Debug)]
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

impl Ast for StatementAst {
    fn get_pos(&self) -> usize {
        match self {
            StatementAst::Assignment(ast) => ast.get_pos(),
            StatementAst::Expression(ast) => ast.get_pos(),
            StatementAst::Let(ast) => ast.get_pos(),
            StatementAst::LoopControlFlow(ast) => ast.get_pos(),
            StatementAst::Pin(ast) => ast.get_pos(),
            StatementAst::Rel(ast) => ast.get_pos(),
            StatementAst::Ret(ast) => ast.get_pos(),
            StatementAst::Use(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            StatementAst::Assignment(ast) => ast.get_final_pos(),
            StatementAst::Expression(ast) => ast.get_final_pos(),
            StatementAst::Let(ast) => ast.get_final_pos(),
            StatementAst::LoopControlFlow(ast) => ast.get_final_pos(),
            StatementAst::Pin(ast) => ast.get_final_pos(),
            StatementAst::Rel(ast) => ast.get_final_pos(),
            StatementAst::Ret(ast) => ast.get_final_pos(),
            StatementAst::Use(ast) => ast.get_final_pos(),
        }
    }
}
