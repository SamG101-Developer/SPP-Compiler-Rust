use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::object_initializer_argument_named::ObjectInitializerArgumentNamedAst;
use crate::spp::asts::object_initializer_argument_unnamed::ObjectInitializerArgumentUnnamedAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum ObjectInitializerArgumentAst {
    Named(ObjectInitializerArgumentNamedAst),
    Unnamed(ObjectInitializerArgumentUnnamedAst),
}
