use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum LambdaExpressionParameterAst {
    Required(FunctionParameterRequiredAst),
    Optional(FunctionParameterOptionalAst),
    Variadic(FunctionParameterVariadicAst),
}
