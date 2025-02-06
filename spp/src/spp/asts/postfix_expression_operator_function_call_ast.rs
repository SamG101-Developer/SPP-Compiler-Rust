use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_call_argument_group_ast::FunctionCallArgumentGroupAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;

#[derive(Clone, Debug)]
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

impl Ast for PostfixExpressionOperatorFunctionCallAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.function_args_group.get_final_pos()
    }
}
