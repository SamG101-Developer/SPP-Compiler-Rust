use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct GenExpressionAst {
    pub pos: usize,
    pub tok_gen: TokenAst,
    pub tok_with: Option<TokenAst>,
    pub convention: Option<ConventionAst>,
    pub expression: Option<Box<ExpressionAst>>,
}

impl GenExpressionAst {
    pub fn new(
        pos: usize,
        tok_gen: TokenAst,
        tok_with: Option<TokenAst>,
        convention: Option<ConventionAst>,
        expression: Option<Box<ExpressionAst>>,
    ) -> Self {
        Self {
            pos,
            tok_gen,
            tok_with,
            convention,
            expression,
        }
    }
}

impl Ast for GenExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        if let Some(expression) = &self.expression {
            expression.get_final_pos()
        } else {
            self.tok_gen.get_final_pos()
        }
    }
}
