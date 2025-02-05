use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::global_constant_ast::GlobalConstantAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;

#[derive(Clone, Debug)]
pub enum ModuleMemberAst {
    Class(ClassPrototypeAst),
    Function(FunctionPrototypeAst),
    SupExtension(SupPrototypeExtensionAst),
    SupFunctions(SupPrototypeFunctionsAst),
    UseStatement(UseStatementAst),
    GlobalConst(GlobalConstantAst),
}

impl Ast for ModuleMemberAst {
    fn get_pos(&self) -> usize {
        match self {
            ModuleMemberAst::Class(ast) => ast.get_pos(),
            ModuleMemberAst::Function(ast) => ast.get_pos(),
            ModuleMemberAst::SupExtension(ast) => ast.get_pos(),
            ModuleMemberAst::SupFunctions(ast) => ast.get_pos(),
            ModuleMemberAst::UseStatement(ast) => ast.get_pos(),
            ModuleMemberAst::GlobalConst(ast) => ast.get_pos(),
        }
    }
}
