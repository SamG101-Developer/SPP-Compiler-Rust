use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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
