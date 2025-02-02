use crate::asts::token_ast::TokenAst;

pub struct PatternVariantDestructureSkip1ArgumentAst {
    pub pos: usize,
    pub tok_underscore: TokenAst,
}

impl PatternVariantDestructureSkip1ArgumentAst {
    pub fn new(pos: usize, tok_underscore: TokenAst) -> Self {
        Self {
            pos,
            tok_underscore,
        }
    }
}
