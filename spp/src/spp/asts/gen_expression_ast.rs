use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct GenExpressionAst {
    pub tok_gen: TokenAst,
    pub tok_with: Option<TokenAst>,
    pub convention: Option<ConventionAst>,
    pub expression: Option<Box<ExpressionAst>>,
}

impl GenExpressionAst {
    pub fn new(tok_gen: TokenAst, tok_with: Option<TokenAst>, convention: Option<ConventionAst>, expression: Option<Box<ExpressionAst>>) -> Self {
        Self { tok_gen, tok_with, convention, expression }
    }
}

impl Ast for GenExpressionAst {
    fn get_pos(&self) -> usize {
        self.tok_gen.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.expression.as_ref().map_or(self.tok_gen.get_final_pos(), |x| x.get_final_pos())
    }
}
