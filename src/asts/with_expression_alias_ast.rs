use crate::asts::local_variable_ast::LocalVariableAst;
use crate::asts::token_ast::TokenAst;

pub struct WithExpressionAliasAst {
    pub pos: usize,
    pub variable: LocalVariableAst,
    pub tok_assign: TokenAst,
}

impl WithExpressionAliasAst {
    pub fn new(pos: usize, variable: LocalVariableAst, tok_assign: TokenAst) -> Self {
        Self {
            pos,
            variable,
            tok_assign,
        }
    }
}
