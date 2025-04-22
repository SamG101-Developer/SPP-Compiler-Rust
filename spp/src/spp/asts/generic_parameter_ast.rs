use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::generic_parameter_comp_optional_ast::GenericParameterCompOptionalAst;
use crate::spp::asts::generic_parameter_comp_required_ast::GenericParameterCompRequiredAst;
use crate::spp::asts::generic_parameter_comp_variadic_ast::GenericParameterCompVariadicAst;
use crate::spp::asts::generic_parameter_type_optional_ast::GenericParameterTypeOptionalAst;
use crate::spp::asts::generic_parameter_type_required_ast::GenericParameterTypeRequiredAst;
use crate::spp::asts::generic_parameter_type_variadic_ast::GenericParameterTypeVariadicAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum GenericParameterAst {
    CompRequired(GenericParameterCompRequiredAst),
    CompOptional(GenericParameterCompOptionalAst),
    CompVariadic(GenericParameterCompVariadicAst),
    TypeRequired(GenericParameterTypeRequiredAst),
    TypeOptional(GenericParameterTypeOptionalAst),
    TypeVariadic(GenericParameterTypeVariadicAst),
}
