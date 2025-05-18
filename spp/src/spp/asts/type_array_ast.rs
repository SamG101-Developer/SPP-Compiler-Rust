use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

pub struct TypeArrayAst {
    pub tok_left_bracket: TokenAst,
    pub type_: TypeAst,
    pub tok_comma: TokenAst,
    pub size: TokenAst,
    pub tok_right_bracket: TokenAst,
}

impl TypeArrayAst {
    pub fn new(tok_left_bracket: TokenAst, type_: TypeAst, tok_comma: TokenAst, size: TokenAst, tok_right_bracket: TokenAst) -> Self {
        Self { tok_left_bracket, type_, tok_comma, size, tok_right_bracket }
    }
}

impl Ast for TypeArrayAst {
    fn get_pos(&self) -> usize {
        self.tok_left_bracket.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_right_bracket.get_final_pos()
    }
}
