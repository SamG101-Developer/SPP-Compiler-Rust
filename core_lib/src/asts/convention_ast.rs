use crate::asts::ast::Ast;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub enum ConventionAst {
    Mov {
        pos: usize,
    },
    Mut {
        pos: usize,
        tok_mut: TokenAst,
        tok_borrow: TokenAst,
    },
    Ref {
        pos: usize,
        tok_borrow: TokenAst,
    },
}

impl ConventionAst {
    pub fn new_mov(pos: usize) -> Self {
        Self::Mov { pos }
    }

    pub fn new_mut(pos: usize, tok_mut: TokenAst, tok_borrow: TokenAst) -> Self {
        Self::Mut {
            pos,
            tok_mut,
            tok_borrow,
        }
    }

    pub fn new_ref(pos: usize, tok_borrow: TokenAst) -> Self {
        Self::Ref { pos, tok_borrow }
    }
}

impl Ast for ConventionAst {
    fn get_pos(&self) -> usize {
        match self {
            ConventionAst::Mov { pos } => *pos,
            ConventionAst::Mut { pos, .. } => *pos,
            ConventionAst::Ref { pos, .. } => *pos,
        }
    }
}
