use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::convention_mut_ast::ConventionMutAst;
use crate::spp::asts::convention_ref_ast::ConventionRefAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum ConventionAst {
    Mut(ConventionMutAst),
    Ref(ConventionRefAst),
}
