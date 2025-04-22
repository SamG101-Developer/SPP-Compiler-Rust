use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::loop_condition_boolean_ast::LoopConditionBooleanAst;
use crate::spp::asts::loop_condition_iterable_ast::LoopConditionIterableAst;

#[derive(Clone, Debug)]
// #[delegation::delegate(derive(Ast))]
pub enum LoopConditionAst {
    Boolean(LoopConditionBooleanAst),
    Iterable(LoopConditionIterableAst),
}
