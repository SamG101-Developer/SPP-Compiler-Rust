use crate::asts::ast::Ast;
use crate::asts::convention_ast::ConventionAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::token_ast::TokenAst;

pub enum FunctionCallArgumentAst {
    NamedArgument {
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        convention: ConventionAst,
        value: ExpressionAst,
    },
    UnnamedArgument {
        pos: usize,
        convention: ConventionAst,
        tok_unpack: Option<TokenAst>,
        value: ExpressionAst,
    },
}

impl FunctionCallArgumentAst {
    pub fn new_named_argument(
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        convention: ConventionAst,
        value: ExpressionAst,
    ) -> Self {
        Self::NamedArgument {
            pos,
            name,
            tok_assign,
            convention,
            value,
        }
    }

    pub fn new_unnamed_argument(
        pos: usize,
        convention: ConventionAst,
        tok_unpack: Option<TokenAst>,
        value: ExpressionAst,
    ) -> Self {
        Self::UnnamedArgument {
            pos,
            convention,
            tok_unpack,
            value,
        }
    }
}

impl Ast for FunctionCallArgumentAst {
    fn get_pos(&self) -> usize {
        match self {
            FunctionCallArgumentAst::NamedArgument { pos, .. } => *pos,
            FunctionCallArgumentAst::UnnamedArgument { pos, .. } => *pos,
        }
    }
}
