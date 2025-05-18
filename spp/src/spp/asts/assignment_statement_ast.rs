use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct AssignmentStatementAst {
    pub lhs: Vec<ExpressionAst>,
    pub op: TokenAst,
    pub rhs: Vec<ExpressionAst>,
}

impl AssignmentStatementAst {
    pub fn new(lhs: Vec<ExpressionAst>, op: TokenAst, rhs: Vec<ExpressionAst>) -> Self {
        Self { lhs, op, rhs }
    }
}

impl Ast for AssignmentStatementAst {
    fn get_pos(&self) -> usize {
        self.lhs[0].get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.rhs.last().unwrap().get_final_pos()
    }
}
