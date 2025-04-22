use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::type_unary_expression_operator_namespace_ast::TypeUnaryExpressionOperatorNamespaceAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum TypeUnaryExpressionOperatorAst {
    Borrow(ConventionAst),
    Namespace(TypeUnaryExpressionOperatorNamespaceAst),
}
