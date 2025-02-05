use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct AssignmentStatementAst {
    pub pos: usize,
    pub lhs: Vec<ExpressionAst>,
    pub op: TokenAst,
    pub rhs: Vec<ExpressionAst>,
}

impl AssignmentStatementAst {
    pub fn new(pos: usize, lhs: Vec<ExpressionAst>, op: TokenAst, rhs: Vec<ExpressionAst>) -> Self {
        Self { pos, lhs, op, rhs }
    }
}

impl Ast for AssignmentStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
