use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct BinaryExpressionAst {
    pub left: Box<ExpressionAst>,
    pub operator: TokenAst,
    pub right: Box<ExpressionAst>,
}

impl BinaryExpressionAst {
    pub fn new(left: Box<ExpressionAst>, operator: TokenAst, right: Box<ExpressionAst>) -> Self {
        Self { left, operator, right }
    }
}

impl Ast for BinaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.left.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.right.get_final_pos()
    }
}
