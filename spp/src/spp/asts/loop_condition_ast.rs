use crate::spp::asts::ast::Ast;
use crate::spp::asts::loop_condition_boolean_ast::LoopConditionBooleanAst;
use crate::spp::asts::loop_condition_iterable_ast::LoopConditionIterableAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum LoopConditionAst {
    Boolean(LoopConditionBooleanAst),
    Iterable(LoopConditionIterableAst),
}
