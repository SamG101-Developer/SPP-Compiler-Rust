use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::cmp_statement_ast::CmpStatementAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum ModuleMemberAst {
    Cls(ClassPrototypeAst),
    Fun(FunctionPrototypeAst),
    Ext(SupPrototypeExtensionAst),
    Sup(SupPrototypeFunctionsAst),
    Use(UseStatementAst),
    Cmp(CmpStatementAst),
}
