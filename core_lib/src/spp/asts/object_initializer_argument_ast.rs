use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub enum ObjectInitializerArgumentAst {
    Named {
        pos: usize,
        name: IdentifierAst,
        tk_assign: TokenAst,
        value: Box<ExpressionAst>,
    },
    Unnamed {
        pos: usize,
        tok_default: Option<TokenAst>,
        name: IdentifierAst,
    },
}

impl ObjectInitializerArgumentAst {
    pub fn new_named(
        pos: usize,
        name: IdentifierAst,
        tk_assign: TokenAst,
        value: Box<ExpressionAst>,
    ) -> Self {
        Self::Named {
            pos,
            name,
            tk_assign,
            value,
        }
    }

    pub fn new_unnamed(pos: usize, tok_default: Option<TokenAst>, name: IdentifierAst) -> Self {
        Self::Unnamed {
            pos,
            tok_default,
            name,
        }
    }
}

impl Ast for ObjectInitializerArgumentAst {
    fn get_pos(&self) -> usize {
        match self {
            ObjectInitializerArgumentAst::Named { pos, .. } => *pos,
            ObjectInitializerArgumentAst::Unnamed { pos, .. } => *pos,
        }
    }
}
