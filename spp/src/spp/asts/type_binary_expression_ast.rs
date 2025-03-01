use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct TypeBinaryExpressionAst {
    pub(crate) pos: usize,
    pub(crate) left: Box<TypeAst>,
    pub(crate) op: TokenAst,
    pub(crate) right: Box<TypeAst>,
}

impl TypeBinaryExpressionAst {
    pub fn new(pos: usize, left: Box<TypeAst>, op: TokenAst, right: Box<TypeAst>) -> Self {
        Self {
            pos,
            left,
            op,
            right,
        }
    }
}

impl Ast for TypeBinaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.right.get_final_pos()
    }
}
