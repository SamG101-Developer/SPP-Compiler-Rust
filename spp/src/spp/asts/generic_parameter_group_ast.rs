use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_ast::GenericParameterAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct GenericParameterGroupAst {
    pub tok_bracket_l: TokenAst,
    pub args: Vec<GenericParameterAst>,
    pub tok_bracket_r: TokenAst,
}

impl GenericParameterGroupAst {
    pub fn new(tok_bracket_l: TokenAst, args: Vec<GenericParameterAst>, tok_bracket_r: TokenAst) -> Self {
        Self { tok_bracket_l, args, tok_bracket_r }
    }
}

impl Ast for GenericParameterGroupAst {
    fn get_pos(&self) -> usize {
        self.tok_bracket_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
