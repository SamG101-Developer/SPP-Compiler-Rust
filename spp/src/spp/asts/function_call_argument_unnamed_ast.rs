use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct FunctionCallArgumentUnnamedAst {
    pos: usize,
    convention: Option<ConventionAst>,
    tok_unpack: Option<TokenAst>,
    value: ExpressionAst,
}

impl FunctionCallArgumentUnnamedAst {
    pub fn new(
        pos: usize,
        convention: Option<ConventionAst>,
        tok_unpack: Option<TokenAst>,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            convention,
            tok_unpack,
            value,
        }
    }
}

impl Ast for FunctionCallArgumentUnnamedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
