use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct TypeParenthesizedExpressionAst {
    pub pos: usize,
    pub left_parenthesis: TokenAst,
    pub expression: Box<TypeAst>,
    pub right_parenthesis: TokenAst,
}

impl TypeParenthesizedExpressionAst {
    pub fn new(
        pos: usize,
        left_parenthesis: TokenAst,
        expression: Box<TypeAst>,
        right_parenthesis: TokenAst,
    ) -> Self {
        Self {
            pos,
            left_parenthesis,
            expression,
            right_parenthesis,
        }
    }
}

impl Ast for TypeParenthesizedExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.right_parenthesis.get_final_pos()
    }
}
