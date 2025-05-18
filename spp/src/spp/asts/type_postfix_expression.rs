use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::generic_argument_ast::GenericArgumentAst;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::mixins::abstract_type_ast::AbstractTypeAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_postfix_expression_operator_ast::TypePostfixExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct TypePostfixExpressionAst {
    pub lhs: Box<TypeAst>,
    pub op: TypePostfixExpressionOperatorAst,
}

impl TypePostfixExpressionAst {
    pub fn new(lhs: Box<TypeAst>, op: TypePostfixExpressionOperatorAst) -> Self {
        Self { lhs, op }
    }
}

impl Ast for TypePostfixExpressionAst {
    fn get_pos(&self) -> usize {
        self.lhs.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.op.get_final_pos()
    }
}

impl AbstractTypeAst for TypePostfixExpressionAst {
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

    fn get_corresponding_generic(&self, that: &TypeAst, generic_parameter_name: &TypeAst) -> TypeAst {
        todo!()
    }

    fn contains_generic(&self, generic_parameter_name: &TypeAst) -> bool {
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
