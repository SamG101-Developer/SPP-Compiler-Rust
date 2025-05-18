use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionAst {
    pub lhs: Box<ExpressionAst>,
    pub op: PostfixExpressionOperatorAst,
}

impl PostfixExpressionAst {
    pub fn new(lhs: Box<ExpressionAst>, op: PostfixExpressionOperatorAst) -> Self {
        Self { lhs, op }
    }
}

impl Ast for PostfixExpressionAst {
    fn get_pos(&self) -> usize {
        self.lhs.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.op.get_final_pos()
    }
}
