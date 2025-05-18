use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::unary_expression_operator_ast::UnaryExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct UnaryExpressionAst {
    pub op: UnaryExpressionOperatorAst,
    pub rhs: Box<ExpressionAst>,
}

impl UnaryExpressionAst {
    pub fn new(op: UnaryExpressionOperatorAst, rhs: Box<ExpressionAst>) -> Self {
        Self { op, rhs }
    }
}

impl Ast for UnaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.op.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.rhs.get_final_pos()
    }
}
