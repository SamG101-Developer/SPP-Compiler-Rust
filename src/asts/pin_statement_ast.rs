use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

pub struct PinStatementAst {
    pub pos: usize,
    pub tok_pin: TokenAst,
    pub expressions: Vec<ExpressionAst>,
}

impl PinStatementAst {
    pub fn new(pos: usize, tok_pin: TokenAst, expressions: Vec<ExpressionAst>) -> Self {
        Self {
            pos,
            tok_pin,
            expressions,
        }
    }
}

impl Ast for PinStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}