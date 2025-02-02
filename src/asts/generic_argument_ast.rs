use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

pub enum GenericArgumentAst {
    CompNamed {
        pos: usize,
        name: TypeAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    },
    CompUnnamed {
        pos: usize,
        value: ExpressionAst,
    },
    TypeNamed {
        pos: usize,
        name: TypeAst,
        tok_assign: TokenAst,
        value: TypeAst,
    },
    TypeUnnamed {
        pos: usize,
        type_: TypeAst,
    },
}

impl GenericArgumentAst {
    pub fn new_comp_named(pos: usize, name: TypeAst, tok_assign: TokenAst, value: ExpressionAst) -> Self {
        Self::CompNamed {
            pos,
            name,
            tok_assign,
            value,
        }
    }

    pub fn new_comp_unnamed(pos: usize, value: ExpressionAst) -> Self {
        Self::CompUnnamed { pos, value }
    }

    pub fn new_type_named(pos: usize, name: TypeAst, tok_assign: TokenAst, value: TypeAst) -> Self {
        Self::TypeNamed {
            pos,
            name,
            tok_assign,
            value,
        }
    }

    pub fn new_type_unnamed(pos: usize, type_: TypeAst) -> Self {
        Self::TypeUnnamed { pos, type_ }
    }
}

impl Ast for GenericArgumentAst {
    fn get_pos(&self) -> usize {
        match self {
            GenericArgumentAst::CompNamed { pos, .. } => *pos,
            GenericArgumentAst::CompUnnamed { pos, .. } => *pos,
            GenericArgumentAst::TypeNamed { pos, .. } => *pos,
            GenericArgumentAst::TypeUnnamed { pos, .. } => *pos,
        }
    }
}
