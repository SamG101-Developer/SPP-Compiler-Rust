use crate::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;

pub enum UnaryExpressionOperatorAst {
    Async(UnaryExpressionOperatorAsyncAst),
}
