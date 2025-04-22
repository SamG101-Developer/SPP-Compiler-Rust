use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use enum_dispatch::enum_dispatch;
use crate::spp::asts::generic_argument_comp_named_ast::GenericArgumentCompNamedAst;
use crate::spp::asts::generic_argument_comp_unnamed_ast::GenericArgumentCompUnnamedAst;
use crate::spp::asts::generic_argument_type_named_ast::GenericArgumentTypeNamedAst;
use crate::spp::asts::generic_argument_type_unnamed_ast::GenericArgumentTypeUnnamedAst;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum GenericArgumentAst {
    CompNamed(GenericArgumentCompNamedAst),
    CompUnnamed(GenericArgumentCompUnnamedAst),
    TypeNamed(GenericArgumentTypeNamedAst),
    TypeUnnamed(GenericArgumentTypeUnnamedAst),
}
