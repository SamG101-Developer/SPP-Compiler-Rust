use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_call_argument::FunctionCallArgumentAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct FunctionCallArgumentGroupAst {
    pub tok_paren_l: TokenAst,
    pub arguments: Vec<FunctionCallArgumentAst>,
    pub tok_paren_r: TokenAst,
}

impl FunctionCallArgumentGroupAst {
    pub fn new(tok_paren_l: TokenAst, arguments: Vec<FunctionCallArgumentAst>, tok_paren_r: TokenAst) -> Self {
        Self { tok_paren_l, arguments, tok_paren_r }
    }
}

impl Ast for FunctionCallArgumentGroupAst {
    fn get_pos(&self) -> usize {
        self.tok_paren_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_paren_r.get_final_pos()
    }
}
