use crate::asts::ast::Ast;
use crate::asts::convention_ast::ConventionAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct GenExpressionAst {
    pub pos: usize,
    pub tok_gen: TokenAst,
    pub tok_with: Option<TokenAst>,
    pub convention: ConventionAst,
    pub expression: Option<Box<ExpressionAst>>,
}

impl GenExpressionAst {
    pub fn new(
        pos: usize,
        tok_gen: TokenAst,
        tok_with: Option<TokenAst>,
        convention: ConventionAst,
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
}
