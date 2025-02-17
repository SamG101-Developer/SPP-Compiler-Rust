use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct PostfixExpressionAst {
    pub pos: usize,
    pub lhs: Box<ExpressionAst>,
    pub op: PostfixExpressionOperatorAst,
}

impl PostfixExpressionAst {
    pub fn new(pos: usize, lhs: Box<ExpressionAst>, op: PostfixExpressionOperatorAst) -> Self {
        Self { pos, lhs, op }
    }
}

impl Ast for PostfixExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.op.get_final_pos()
    }
}
