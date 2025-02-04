use crate::asts::ast::Ast;
use crate::asts::generic_argument_ast::GenericArgumentAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct GenericArgumentGroupAst {
    pub pos: usize,
    pub tok_bracket_l: TokenAst,
    pub args: Vec<GenericArgumentAst>,
    pub tok_bracket_r: TokenAst,
}

impl GenericArgumentGroupAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        arguments: Vec<GenericArgumentAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            args: arguments,
            tok_bracket_r,
        }
    }
}

impl Default for GenericArgumentGroupAst {
    fn default() -> Self {
        Self {
            pos: 0,
            tok_bracket_l: TokenAst::default(),
            args: Vec::new(),
            tok_bracket_r: TokenAst::default(),
        }
    }
}

impl Ast for GenericArgumentGroupAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
