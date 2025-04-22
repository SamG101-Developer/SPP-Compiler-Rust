use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::use_statement_alias_ast::UseStatementAliasAst;
use crate::spp::asts::use_statement_redux_ast::UseStatementReduxAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum UseStatementAst {
    Alias(UseStatementAliasAst),
    Redux(UseStatementReduxAst),
}
