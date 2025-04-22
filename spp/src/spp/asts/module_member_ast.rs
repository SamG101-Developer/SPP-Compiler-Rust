use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::cmp_statement_ast::CmpStatementAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum ModuleMemberAst {
    Cls(ClassPrototypeAst),
    Fun(FunctionPrototypeAst),
    Ext(SupPrototypeExtensionAst),
    Sup(SupPrototypeFunctionsAst),
    Use(UseStatementAst),
    Cmp(CmpStatementAst),
}
