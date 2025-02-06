use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableNestedForDestructureArrayAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
