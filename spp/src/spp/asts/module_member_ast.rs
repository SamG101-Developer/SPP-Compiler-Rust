use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::cmp_statement_ast::CmpStatementAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum ModuleMemberAst {
    Cls(Rc<RefCell<ClassPrototypeAst>>),
    Fun(Rc<RefCell<FunctionPrototypeAst>>),
    Ext(Rc<RefCell<SupPrototypeExtensionAst>>),
    Sup(Rc<RefCell<SupPrototypeFunctionsAst>>),
    Use(Rc<RefCell<UseStatementAst>>),
    Cmp(Rc<RefCell<CmpStatementAst>>),
}

impl Ast for ModuleMemberAst {
    fn get_pos(&self) -> usize {
        match self {
            ModuleMemberAst::Cls(cls) => cls.borrow().get_pos(),
            ModuleMemberAst::Fun(fun) => fun.borrow().get_pos(),
            ModuleMemberAst::Ext(ext) => ext.borrow().get_pos(),
            ModuleMemberAst::Sup(sup) => sup.borrow().get_pos(),
            ModuleMemberAst::Use(use_stmt) => use_stmt.borrow().get_pos(),
            ModuleMemberAst::Cmp(cmp) => cmp.borrow().get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            ModuleMemberAst::Cls(cls) => cls.borrow().get_final_pos(),
            ModuleMemberAst::Fun(fun) => fun.borrow().get_final_pos(),
            ModuleMemberAst::Ext(ext) => ext.borrow().get_final_pos(),
            ModuleMemberAst::Sup(sup) => sup.borrow().get_final_pos(),
            ModuleMemberAst::Use(use_stmt) => use_stmt.borrow().get_final_pos(),
            ModuleMemberAst::Cmp(cmp) => cmp.borrow().get_final_pos(),
        }
    }
}
