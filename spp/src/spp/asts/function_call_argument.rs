use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::function_call_argument_named_ast::FunctionCallArgumentNamedAst;
use crate::spp::asts::function_call_argument_unnamed_ast::FunctionCallArgumentUnnamedAst;

#[derive(Clone, Debug)]
// #[delegation::delegate(derive(Ast))]
pub enum FunctionCallArgumentAst {
    Named(FunctionCallArgumentNamedAst),
    Unnamed(FunctionCallArgumentUnnamedAst),
}
