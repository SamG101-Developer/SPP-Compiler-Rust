use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_member_ast::FunctionMemberAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct FunctionImplementationAst {
    pub pos: usize,
    pub tok_brace_l: TokenAst,
    pub members: Vec<FunctionMemberAst>,
    pub tok_brace_r: TokenAst,
}

impl FunctionImplementationAst {
    pub fn new(
        pos: usize,
        tok_brace_l: TokenAst,
        members: Vec<FunctionMemberAst>,
        tok_brace_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_brace_l,
            members,
            tok_brace_r,
        }
    }
}

impl Ast for FunctionImplementationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_brace_r.get_final_pos()
    }
}
