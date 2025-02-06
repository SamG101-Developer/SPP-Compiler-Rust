use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

impl Ast for PatternVariantDestructureSkip1ArgumentAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
