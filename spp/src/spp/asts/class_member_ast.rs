use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::class_attribute_ast::ClassAttributeAst;

#[derive(Clone, Debug)]
// #[delegation::delegate(derive(Ast))]
pub enum ClassMemberAst {
    Attr(ClassAttributeAst),
}
