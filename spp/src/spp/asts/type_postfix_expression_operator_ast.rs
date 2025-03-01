use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_postfix_expression_operator_indexed_ast::TypePostfixExpressionOperatorIndexedAst;
use crate::spp::asts::type_postfix_expression_operator_nested_ast::TypePostfixExpressionOperatorNestedAst;
use crate::spp::asts::type_postfix_expression_operator_optional_ast::TypePostfixExpressionOperatorOptionalAst;

#[derive(Clone, Debug)]
pub enum TypePostfixExpressionOperatorAst {
    Nested(TypePostfixExpressionOperatorNestedAst),
    Indexed(TypePostfixExpressionOperatorIndexedAst),
    Optional(TypePostfixExpressionOperatorOptionalAst),
}

impl Ast for TypePostfixExpressionOperatorAst {
    fn get_pos(&self) -> usize {
        match self {
            TypePostfixExpressionOperatorAst::Nested(ast) => ast.get_pos(),
            TypePostfixExpressionOperatorAst::Indexed(ast) => ast.get_pos(),
            TypePostfixExpressionOperatorAst::Optional(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            TypePostfixExpressionOperatorAst::Nested(ast) => ast.get_final_pos(),
            TypePostfixExpressionOperatorAst::Indexed(ast) => ast.get_final_pos(),
            TypePostfixExpressionOperatorAst::Optional(ast) => ast.get_final_pos(),
        }
    }
}
