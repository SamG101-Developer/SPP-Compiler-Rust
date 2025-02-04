use crate::asts::ast::Ast;
use crate::asts::class_prototype_ast::ClassPrototypeAst;
use crate::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::asts::global_constant_ast::GlobalConstantAst;
use crate::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::asts::use_statement_ast::UseStatementAst;

#[derive(Clone)]
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
