use crate::spp::asts::ast::Ast;
use crate::spp::asts::class_member_ast::ClassMemberAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct ClassImplementationAst {
    pub pos: usize,
    pub tok_brace_l: TokenAst,
    pub members: Vec<ClassMemberAst>,
    pub tok_brace_r: TokenAst,
}

impl ClassImplementationAst {
    pub fn new(
        pos: usize,
        tok_brace_l: TokenAst,
        members: Vec<ClassMemberAst>,
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

impl Ast for ClassImplementationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_brace_r.get_final_pos()
    }
}
