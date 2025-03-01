use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_unary_expression_operator_namespace_ast::TypeUnaryExpressionOperatorNamespaceAst;

#[derive(Clone, Debug)]
pub enum TypeUnaryExpressionOperatorAst {
    Namespace(TypeUnaryExpressionOperatorNamespaceAst),
}

impl Ast for TypeUnaryExpressionOperatorAst {
    fn get_pos(&self) -> usize {
        match self {
            TypeUnaryExpressionOperatorAst::Namespace(ast) => ast.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            TypeUnaryExpressionOperatorAst::Namespace(ast) => ast.get_final_pos(),
        }
    }
}
