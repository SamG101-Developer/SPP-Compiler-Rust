use crate::spp::asts::ast::Ast;
use crate::spp::asts::use_statement_alias_ast::UseStatementAliasAst;
use crate::spp::asts::use_statement_redux_ast::UseStatementReduxAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum UseStatementAst {
    Alias(UseStatementAliasAst),
    Redux(UseStatementReduxAst),
}
