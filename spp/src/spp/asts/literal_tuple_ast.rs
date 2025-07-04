use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LiteralTupleAst {
    pos: usize,
    tok_parenthesis_l: TokenAst,
    elements: Vec<ExpressionAst>,
    tok_parenthesis_r: TokenAst,
}

impl LiteralTupleAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for LiteralTupleAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
