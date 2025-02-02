use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

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
