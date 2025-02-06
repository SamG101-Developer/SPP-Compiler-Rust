use crate::spp::asts::ast::Ast;
use crate::spp::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::spp::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::spp::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::spp::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;
use crate::spp::asts::postfix_expression_operator_step_keyword_ast::PostfixExpressionOperatorStepKeywordAst;

#[derive(Clone, Debug)]
pub enum PostfixExpressionOperatorAst {
    EarlyReturn(PostfixExpressionOperatorEarlyReturnAst),
    FunctionCall(PostfixExpressionOperatorFunctionCallAst),
    MemberAccess(PostfixExpressionOperatorMemberAccessAst),
    NotKeyword(PostfixExpressionOperatorNotKeywordAst),
    StepKeyword(PostfixExpressionOperatorStepKeywordAst),
}

impl Ast for PostfixExpressionOperatorAst {
    fn get_pos(&self) -> usize {
        match self {
            PostfixExpressionOperatorAst::EarlyReturn(ast) => ast.get_pos(),
            PostfixExpressionOperatorAst::FunctionCall(ast) => ast.get_pos(),
            PostfixExpressionOperatorAst::MemberAccess(ast) => ast.get_pos(),
            PostfixExpressionOperatorAst::NotKeyword(ast) => ast.get_pos(),
            PostfixExpressionOperatorAst::StepKeyword(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PostfixExpressionOperatorAst::EarlyReturn(ast) => ast.get_final_pos(),
            PostfixExpressionOperatorAst::FunctionCall(ast) => ast.get_final_pos(),
            PostfixExpressionOperatorAst::MemberAccess(ast) => ast.get_final_pos(),
            PostfixExpressionOperatorAst::NotKeyword(ast) => ast.get_final_pos(),
            PostfixExpressionOperatorAst::StepKeyword(ast) => ast.get_final_pos(),
        }
    }
}
