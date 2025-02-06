use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_parameter_ast::FunctionParameterAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterGroupAst {
    pub pos: usize,
    pub tok_paren_l: TokenAst,
    pub parameters: Vec<FunctionParameterAst>,
    pub tok_paren_r: TokenAst,
}

impl FunctionParameterGroupAst {
    pub fn new(
        pos: usize,
        tok_paren_l: TokenAst,
        parameters: Vec<FunctionParameterAst>,
        tok_paren_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_paren_l,
            parameters,
            tok_paren_r,
        }
    }
}

impl Ast for FunctionParameterGroupAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
