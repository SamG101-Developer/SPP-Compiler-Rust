use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct TypeBinaryExpressionAst {
    pub left: Box<TypeAst>,
    pub op: TokenAst,
    pub right: Box<TypeAst>,
}

impl TypeBinaryExpressionAst {
    pub fn new(left: Box<TypeAst>, op: TokenAst, right: Box<TypeAst>) -> Self {
        Self { left, op, right }
    }
}

impl Ast for TypeBinaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.left.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.right.get_final_pos()
    }
}
