use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::spp::asts::sup_cmp_statement_ast::SupCmpStatementAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_use_statement_ast::SupUseStatementAst;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum SupMemberAst {
    Fun(Rc<RefCell<FunctionPrototypeAst>>),
    Use(Rc<RefCell<SupUseStatementAst>>),
    Cmp(Rc<RefCell<SupCmpStatementAst>>),
    Ext(Rc<RefCell<SupPrototypeExtensionAst>>),
}

impl Ast for SupMemberAst {
    fn get_pos(&self) -> usize {
        match self {
            SupMemberAst::Fun(fun) => fun.borrow().get_pos(),
            SupMemberAst::Use(use_stmt) => use_stmt.borrow().get_pos(),
            SupMemberAst::Cmp(cmp) => cmp.borrow().get_pos(),
            SupMemberAst::Ext(ext) => ext.borrow().get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            SupMemberAst::Fun(fun) => fun.borrow().get_final_pos(),
            SupMemberAst::Use(use_stmt) => use_stmt.borrow().get_final_pos(),
            SupMemberAst::Cmp(cmp) => cmp.borrow().get_final_pos(),
            SupMemberAst::Ext(ext) => ext.borrow().get_final_pos(),
        }
    }
}
