use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LiteralArrayAst {
    pos: usize,
    tok_bracket_l: TokenAst,
    elements: Vec<ExpressionAst>,
    tok_bracket_r: TokenAst,
}

impl LiteralArrayAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            elements,
            tok_bracket_r,
        }
    }
}

impl Ast for LiteralArrayAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
