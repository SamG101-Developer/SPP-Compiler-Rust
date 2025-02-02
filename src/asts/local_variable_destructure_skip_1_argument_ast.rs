use crate::asts::token_ast::TokenAst;

pub struct LocalVariableDestructureSkip1ArgumentAst {
    pos: usize,
    tok_underscore: TokenAst,
}

impl LocalVariableDestructureSkip1ArgumentAst {
    pub fn new(pos: usize, tok_underscore: TokenAst) -> Self {
        Self {
            pos,
            tok_underscore,
        }
    }
}
