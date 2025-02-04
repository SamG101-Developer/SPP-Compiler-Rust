use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::with_expression_alias_ast::WithExpressionAliasAst;

#[derive(Clone, Debug)]
pub struct WithExpressionAst {
    pub pos: usize,
    pub tok_with: TokenAst,
    pub alias: Option<WithExpressionAliasAst>,
    pub expression: Box<ExpressionAst>,
    body: InnerScopeAst,
}

impl WithExpressionAst {
    pub fn new(
        pos: usize,
        tok_with: TokenAst,
        alias: Option<WithExpressionAliasAst>,
        expression: Box<ExpressionAst>,
        body: InnerScopeAst,
    ) -> Self {
        Self {
            pos,
            tok_with,
            alias,
            expression,
            body,
        }
    }
}

impl Ast for WithExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
