use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct FunctionCallArgumentNamedAst {
    pos: usize,
    name: IdentifierAst,
    tok_assign: TokenAst,
    convention: Option<ConventionAst>,
    value: ExpressionAst,
}

impl FunctionCallArgumentNamedAst {
    pub fn new(
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        convention: Option<ConventionAst>,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            convention,
            value,
        }
    }
}

impl Ast for FunctionCallArgumentNamedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
