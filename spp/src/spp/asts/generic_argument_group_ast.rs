use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_argument_ast::GenericArgumentAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct GenericArgumentGroupAst {
    pub tok_bracket_l: TokenAst,
    pub args: Vec<GenericArgumentAst>,
    pub tok_bracket_r: TokenAst,
}

impl GenericArgumentGroupAst {
    pub fn new(tok_bracket_l: TokenAst, arguments: Vec<GenericArgumentAst>, tok_bracket_r: TokenAst) -> Self {
        Self { tok_bracket_l, args: arguments, tok_bracket_r }
    }
}

impl Ast for GenericArgumentGroupAst {
    fn get_pos(&self) -> usize {
        self.tok_bracket_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
