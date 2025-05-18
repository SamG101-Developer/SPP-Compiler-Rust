use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::function_prototype_ast::FunctionPrototypeBaseAst;
use crate::spp::asts::module_prototype_ast::ModulePrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub enum PreProcessingContext {
    Cls(Box<ClassPrototypeAst>),
    Ext(Box<SupPrototypeExtensionAst>),
    Fun(Box<FunctionPrototypeBaseAst>),
    Mod(Box<ModulePrototypeAst>),
}

#[delegation::delegate]
pub trait Ast {
    fn get_pos(&self) -> usize;

    fn get_final_pos(&self) -> usize;

    fn get_size(&self) -> usize {
        self.get_final_pos() - self.get_pos()
    }
}

pub trait ToBinaryExpression {
    fn to_binary_expression(lhs: ExpressionAst, op: TokenAst, rhs: Self) -> ExpressionAst;
}
