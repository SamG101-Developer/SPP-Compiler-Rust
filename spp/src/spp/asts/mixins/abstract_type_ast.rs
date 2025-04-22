use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::generic_argument_ast::GenericArgumentAst;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::type_ast::TypeAst;
use std::cell::RefCell;
use std::rc::Rc;
use crate::spp::asts::ast::Ast;

// #[delegation::delegate]
pub trait AbstractTypeAst {
    fn type_parts(&self) -> Vec<GenericIdentifierAst>;

    fn namespace_parts(&self) -> Vec<IdentifierAst>;

    fn without_generics(&self) -> TypeAst;

    fn sub_generics(&self, generic_arguments: Vec<GenericArgumentAst>) -> TypeAst;

    fn get_corresponding_generic(
        &self,
        that: &TypeAst,
        generic_parameter_name: &TypeAst,
    ) -> TypeAst;

    fn contains_generic(&self, generic_parameter_name: &TypeAst) -> bool;

    fn symbolic_eq(
        &self,
        that: &TypeAst,
        self_scope: &Scope,
        that_scope: &Scope,
        check_variant: bool,
    ) -> bool;

    fn split_to_scope_and_type(&self, scope: &Scope) -> (Rc<RefCell<Scope>>, TypeAst);

    fn get_convention(&self) -> ConventionAst;

    fn with_convention(&mut self) -> Self;

    fn without_convention(&mut self) -> Self;
}
