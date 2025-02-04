use crate::asts::ast::Ast;
use crate::asts::local_variable_ast::LocalVariableNestedForDestructureArrayAst;
use crate::asts::token_ast::TokenAst;

pub struct LocalVariableDestructureArrayAst {
    pub pos: usize,
    pub tok_bracket_l: TokenAst,
    pub elements: Vec<LocalVariableNestedForDestructureArrayAst>,
    pub tok_bracket_r: TokenAst,
}

impl LocalVariableDestructureArrayAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        elements: Vec<LocalVariableNestedForDestructureArrayAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            elements,
            tok_bracket_r,
        }
    }
}

impl Ast for LocalVariableDestructureArrayAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
