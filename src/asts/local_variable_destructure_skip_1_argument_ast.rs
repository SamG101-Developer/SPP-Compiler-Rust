use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
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

impl Ast for LocalVariableDestructureSkip1ArgumentAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
