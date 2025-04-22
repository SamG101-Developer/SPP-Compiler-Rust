use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::generic_argument_ast::GenericArgumentAst;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::mixins::abstract_type_ast::AbstractTypeAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_unary_expression_operator_ast::TypeUnaryExpressionOperatorAst;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TypeUnaryExpressionAst {
    pub pos: usize,
    pub op: TypeUnaryExpressionOperatorAst,
    pub rhs: Box<TypeAst>,
}

impl TypeUnaryExpressionAst {
    pub fn new(pos: usize, op: TypeUnaryExpressionOperatorAst, rhs: Box<TypeAst>) -> Self {
        Self { pos, op, rhs }
    }
}

impl Ast for TypeUnaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.rhs.get_final_pos()
    }
}

impl AbstractTypeAst for TypeUnaryExpressionAst {
    fn type_parts(&self) -> Vec<GenericIdentifierAst> {
        todo!()
    }

    fn namespace_parts(&self) -> Vec<IdentifierAst> {
        todo!()
    }

    fn without_generics(&self) -> TypeAst {
        todo!()
    }

    fn sub_generics(&self, generic_arguments: Vec<GenericArgumentAst>) -> TypeAst {
        todo!()
    }

    fn get_corresponding_generic(
        &self,
        that: &TypeAst,
        generic_parameter_name: &TypeAst,
    ) -> TypeAst {
        todo!()
    }

    fn contains_generic(&self, generic_parameter_name: &TypeAst) -> bool {
        todo!()
    }

    fn symbolic_eq(
        &self,
        that: &TypeAst,
        self_scope: &Scope,
        that_scope: &Scope,
        check_variant: bool,
    ) -> bool {
        todo!()
    }

    fn split_to_scope_and_type(&self, scope: &Scope) -> (Rc<RefCell<Scope>>, TypeAst) {
        todo!()
    }

    fn get_convention(&self) -> ConventionAst {
        todo!()
    }

    fn with_convention(&mut self) -> Self {
        todo!()
    }

    fn without_convention(&mut self) -> Self {
        todo!()
    }
}
