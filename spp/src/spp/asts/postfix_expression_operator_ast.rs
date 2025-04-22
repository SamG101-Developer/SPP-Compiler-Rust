use crate::spp::asts::ast::Ast;
use crate::spp::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::spp::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::spp::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::spp::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PostfixExpressionOperatorAst {
    EarlyReturn(PostfixExpressionOperatorEarlyReturnAst),
    FunctionCall(PostfixExpressionOperatorFunctionCallAst),
    MemberAccess(PostfixExpressionOperatorMemberAccessAst),
    NotKeyword(PostfixExpressionOperatorNotKeywordAst),
}
