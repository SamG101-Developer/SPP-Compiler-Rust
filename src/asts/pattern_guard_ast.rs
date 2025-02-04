use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct PatternGuardAst {
    pub pos: usize,
    pub tok_guard: TokenAst,
    pub expression: ExpressionAst,
}

impl PatternGuardAst {
    pub fn new(pos: usize, tok_guard: TokenAst, expression: ExpressionAst) -> Self {
        Self {
            pos,
            tok_guard,
            expression,
        }
    }
}

impl Ast for PatternGuardAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
