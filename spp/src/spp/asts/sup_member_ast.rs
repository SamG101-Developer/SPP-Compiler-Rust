use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::sup_cmp_statement_ast::SupCmpStatementAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_use_statement_ast::SupUseStatementAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum SupMemberAst {
    Fun(FunctionPrototypeAst),
    Use(SupUseStatementAst),
    Cmp(SupCmpStatementAst),
    Ext(SupPrototypeExtensionAst),
}
