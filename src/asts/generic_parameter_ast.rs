use crate::asts::generic_parameter_comp_optional_ast::GenericParameterCompOptionalAst;
use crate::asts::generic_parameter_comp_required::GenericParameterCompRequiredAst;
use crate::asts::generic_parameter_comp_variadic_ast::GenericParameterCompVariadicAst;
use crate::asts::generic_parameter_type_optional::GenericParameterTypeOptionalAst;
use crate::asts::generic_parameter_type_required::GenericParameterTypeRequiredAst;
use crate::asts::generic_parameter_type_variadic::GenericParameterTypeVariadicAst;

pub enum GenericParameterAst {
    CompRequired(GenericParameterCompRequiredAst),
    CompOptional(GenericParameterCompOptionalAst),
    CompVariadic(GenericParameterCompVariadicAst),
    TypeRequired(GenericParameterTypeRequiredAst),
    TypeOptional(GenericParameterTypeOptionalAst),
    TypeVariadic(GenericParameterTypeVariadicAst),
}
