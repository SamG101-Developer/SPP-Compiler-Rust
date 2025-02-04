use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone)]
pub struct GenericParameterConstraintsAst {
    pub pos: usize,
    pub tok_colon: TokenAst,
    pub constraints: Vec<TypeAst>,
}

impl GenericParameterConstraintsAst {
    pub fn new(pos: usize, tok_colon: TokenAst, constraints: Vec<TypeAst>) -> Self {
        Self {
            pos,
            tok_colon,
            constraints,
        }
    }
}

impl Ast for GenericParameterConstraintsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
