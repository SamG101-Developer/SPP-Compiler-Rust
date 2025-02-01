use crate::asts::function_call_argument_group_ast::FunctionCallArgumentGroupAst;
use crate::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;

pub struct PostfixExpressionOperatorFunctionCallAst {
    pub pos: usize,
    pub generic_args_group: Option<GenericArgumentGroupAst>,
    pub function_args_group: FunctionCallArgumentGroupAst,
    pub tok_fold: Option<TokenAst>,

    _overload: Option<(Scope, FunctionPrototypeAst)>,
    _is_async: Option<UnaryExpressionOperatorAsyncAst>,
}

impl PostfixExpressionOperatorFunctionCallAst {
    pub fn new(
        pos: usize,
        generic_args_group: Option<GenericArgumentGroupAst>,
        function_args_group: FunctionCallArgumentGroupAst,
        tok_fold: Option<TokenAst>,
    ) -> Self {
        Self {
            pos,
            generic_args_group,
            function_args_group,
            tok_fold,
            _overload: None,
            _is_async: None,
        }
    }
}
