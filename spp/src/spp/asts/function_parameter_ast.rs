use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;

#[derive(Clone, Debug)]
// #[delegation::delegate(derive(Ast))]
pub enum FunctionParameterAst {
    // No default because function parameters are always Vec<FunctionParameterAst>
    Self_(FunctionParameterSelfAst),
    Required(FunctionParameterRequiredAst),
    Optional(FunctionParameterOptionalAst),
    Variadic(FunctionParameterVariadicAst),
}
