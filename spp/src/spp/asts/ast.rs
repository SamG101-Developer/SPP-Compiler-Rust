use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeBaseAst;
use crate::spp::asts::module_prototype_ast::ModulePrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::token_ast::TokenAst;
use enum_dispatch::enum_dispatch;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum PreProcessingContext {
    Cls(Rc<RefCell<ClassPrototypeAst>>),
    Ext(Rc<RefCell<SupPrototypeExtensionAst>>),
    Fun(Rc<RefCell<FunctionPrototypeBaseAst>>),
    Mod(Rc<RefCell<ModulePrototypeAst>>),
}

#[enum_dispatch]
pub trait Ast {
    fn get_pos(&self) -> usize;

    fn get_final_pos(&self) -> usize;

    fn get_size(&self) -> usize {
        self.get_final_pos() - self.get_pos()
    }

    fn stage_1_preprocess_asts(
        &mut self,
        context: PreProcessingContext,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn stage_2_generate_top_level_scopes(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn stage_3_generate_top_level_aliases(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn stage_4_qualify_types(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
    fn stage_5_load_super_scopes(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn stage_6_pre_analysis(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }

    fn stage_8_analyse_semantics(
        &mut self,
        scope_manager: &mut ScopeManager,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
}

pub trait ToBinaryExpression {
    fn to_binary_expression(
        pos: usize,
        lhs: ExpressionAst,
        op: TokenAst,
        rhs: Self,
    ) -> ExpressionAst;
}
