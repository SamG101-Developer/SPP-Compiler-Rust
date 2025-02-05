use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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