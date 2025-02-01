use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

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
