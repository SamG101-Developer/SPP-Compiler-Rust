use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct FunctionCallArgumentUnnamedAst {
    convention: Option<ConventionAst>,
    tok_unpack: Option<TokenAst>,
    value: ExpressionAst,
}

impl FunctionCallArgumentUnnamedAst {
    pub fn new(convention: Option<ConventionAst>, tok_unpack: Option<TokenAst>, value: ExpressionAst) -> Self {
        Self { convention, tok_unpack, value }
    }
}

impl Ast for FunctionCallArgumentUnnamedAst {
    fn get_pos(&self) -> usize {
        self.convention.as_ref().map_or(self.tok_unpack.as_ref().map_or(self.value.get_pos(), |x| x.get_pos()), |x| x.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
