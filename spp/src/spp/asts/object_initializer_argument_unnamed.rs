use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ObjectInitializerArgumentUnnamedAst {
    tok_default: Option<TokenAst>,
    name: ExpressionAst,
}

impl ObjectInitializerArgumentUnnamedAst {
    pub fn new(tok_default: Option<TokenAst>, name: ExpressionAst) -> Self {
        Self { tok_default, name }
    }
}

impl Ast for ObjectInitializerArgumentUnnamedAst {
    fn get_pos(&self) -> usize {
        self.tok_default.as_ref().map_or(self.name.get_pos(), |t| t.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.name.get_final_pos()
    }
}
