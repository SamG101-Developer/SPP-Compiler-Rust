use crate::spp::asts::ast::Ast;
use crate::spp::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;

#[derive(Clone, Debug)]
pub enum UnaryExpressionOperatorAst {
    Async(UnaryExpressionOperatorAsyncAst),
}

impl Ast for UnaryExpressionOperatorAst {
    fn get_pos(&self) -> usize {
        match self {
            UnaryExpressionOperatorAst::Async(ast) => ast.get_pos(),
        }
    }
}
