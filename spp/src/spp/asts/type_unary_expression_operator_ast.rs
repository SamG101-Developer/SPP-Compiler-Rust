use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_unary_expression_operator_namespace_ast::TypeUnaryExpressionOperatorNamespaceAst;
use crate::spp::asts::convention_ast::ConventionAst;

#[derive(Clone, Debug)]
pub enum TypeUnaryExpressionOperatorAst {
    Borrow(ConventionAst),
    Namespace(TypeUnaryExpressionOperatorNamespaceAst),
}

impl Ast for TypeUnaryExpressionOperatorAst {
    fn get_pos(&self) -> usize {
        match self {
            TypeUnaryExpressionOperatorAst::Borrow(ast) => ast.get_pos(),
            TypeUnaryExpressionOperatorAst::Namespace(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            TypeUnaryExpressionOperatorAst::Borrow(ast) => ast.get_final_pos(),
            TypeUnaryExpressionOperatorAst::Namespace(ast) => ast.get_final_pos(),
        }
    }
}
