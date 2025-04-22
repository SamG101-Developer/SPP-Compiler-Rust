use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum FunctionParameterAst {
    Self_(FunctionParameterSelfAst),
    Required(FunctionParameterRequiredAst),
    Optional(FunctionParameterOptionalAst),
    Variadic(FunctionParameterVariadicAst),
}
