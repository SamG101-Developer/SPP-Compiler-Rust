use crate::spp::asts::ast::Ast;
use crate::spp::asts::let_statement_initialized_ast::LetStatementInitializedAst;
use crate::spp::asts::let_statement_uninitialized_ast::LetStatementUninitializedAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum LetStatementAst {
    Initialized(LetStatementInitializedAst),
    Uninitialized(LetStatementUninitializedAst),
}
