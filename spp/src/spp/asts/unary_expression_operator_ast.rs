use crate::spp::asts::ast::Ast;
use crate::spp::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum UnaryExpressionOperatorAst {
    Async(UnaryExpressionOperatorAsyncAst),
}
