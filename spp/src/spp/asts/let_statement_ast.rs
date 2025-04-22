use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::let_statement_initialized_ast::LetStatementInitializedAst;
use crate::spp::asts::let_statement_uninitialized_ast::LetStatementUninitializedAst;

#[derive(Clone, Debug)]
// #[delegation::delegate(derive(Ast))]
pub enum LetStatementAst {
    Initialized(LetStatementInitializedAst),
    Uninitialized(LetStatementUninitializedAst),
}
