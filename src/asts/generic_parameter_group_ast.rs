use crate::asts::ast::Ast;
use crate::asts::generic_parameter_ast::GenericParameterAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct GenericParameterGroupAst {
    pub pos: usize,
    pub tok_bracket_l: TokenAst,
    pub args: Vec<GenericParameterAst>,
    pub tok_bracket_r: TokenAst,
}

impl GenericParameterGroupAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        args: Vec<GenericParameterAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            args,
            tok_bracket_r,
        }
    }
}

impl Ast for GenericParameterGroupAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
