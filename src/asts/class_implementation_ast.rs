use crate::asts::ast::Ast;
use crate::asts::class_member_ast::ClassMemberAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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
}
