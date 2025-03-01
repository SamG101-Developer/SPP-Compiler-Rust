use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_postfix_expression::TypePostfixExpressionAst;
use crate::spp::asts::type_single_ast::TypeSingleAst;
use crate::spp::asts::type_unary_expression_ast::TypeUnaryExpressionAst;

#[derive(Clone, Debug)]
pub enum TypeAst {
    Postfix(TypePostfixExpressionAst),
    Unary(TypeUnaryExpressionAst),
    Single(TypeSingleAst),
}

impl Ast for TypeAst {
    fn get_pos(&self) -> usize {
        match self {
            TypeAst::Postfix(ast) => ast.get_pos(),
            TypeAst::Unary(ast) => ast.get_pos(),
            TypeAst::Single(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            TypeAst::Postfix(ast) => ast.get_final_pos(),
            TypeAst::Unary(ast) => ast.get_final_pos(),
            TypeAst::Single(ast) => ast.get_final_pos(),
        }
    }
}
