use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::spp::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::spp::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::spp::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum PostfixExpressionOperatorAst {
    EarlyReturn(PostfixExpressionOperatorEarlyReturnAst),
    FunctionCall(PostfixExpressionOperatorFunctionCallAst),
    MemberAccess(PostfixExpressionOperatorMemberAccessAst),
    NotKeyword(PostfixExpressionOperatorNotKeywordAst),
}
