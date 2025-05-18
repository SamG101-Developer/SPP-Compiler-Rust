use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct ObjectInitializerArgumentNamedAst {
    name: IdentifierAst,
    tk_assign: TokenAst,
    value: Box<ExpressionAst>,
}

impl ObjectInitializerArgumentNamedAst {
    pub fn new(name: IdentifierAst, tk_assign: TokenAst, value: Box<ExpressionAst>) -> Self {
        Self { name, tk_assign, value }
    }
}

impl Ast for ObjectInitializerArgumentNamedAst {
    fn get_pos(&self) -> usize {
        self.name.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
