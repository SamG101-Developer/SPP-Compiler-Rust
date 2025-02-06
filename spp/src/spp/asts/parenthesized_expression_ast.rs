use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ParenthesizedExpressionAst {
    pos: usize,
    tok_parenthesis_l: TokenAst,
    expr: Box<ExpressionAst>,
    tok_parenthesis_r: TokenAst,
}

impl ParenthesizedExpressionAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        expr: Box<ExpressionAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_l,
            expr,
            tok_parenthesis_r,
        }
    }
}

impl Ast for ParenthesizedExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
