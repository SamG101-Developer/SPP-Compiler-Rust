use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_postfix_expression_operator_nested_ast::TypePostfixExpressionOperatorNestedAst;
use crate::spp::asts::type_postfix_expression_operator_optional_ast::TypePostfixExpressionOperatorOptionalAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum TypePostfixExpressionOperatorAst {
    Nested(TypePostfixExpressionOperatorNestedAst),
    Optional(TypePostfixExpressionOperatorOptionalAst),
}
