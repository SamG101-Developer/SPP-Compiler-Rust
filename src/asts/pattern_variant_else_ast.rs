use crate::asts::token_ast::TokenAst;

pub struct PatternVariantElseAst {
    pub pos: usize,
    pub tok_else: TokenAst,
}

impl PatternVariantElseAst {
    pub fn new(pos: usize, tok_else: TokenAst) -> Self {
        Self { pos, tok_else }
    }
}
