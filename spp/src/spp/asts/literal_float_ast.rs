use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LiteralFloatAst {
    tok_sign: Option<TokenAst>,
    integer_value: TokenAst,
    tok_dot: TokenAst,
    float_value: TokenAst,
    type_: Option<TypeAst>,
}

impl LiteralFloatAst {
    pub fn new(tok_sign: Option<TokenAst>, integer_value: TokenAst, tok_dot: TokenAst, float_value: TokenAst, type_: Option<TypeAst>) -> Self {
        Self { tok_sign, integer_value, tok_dot, float_value, type_, }
    }
}

impl Ast for LiteralFloatAst {
    fn get_pos(&self) -> usize {
        self.tok_sign.as_ref().map_or(self.integer_value.get_pos(), |tok| tok.get_pos())
    }

    fn get_final_pos(&self) -> usize {
        self.type_.as_ref().map_or(self.float_value.get_final_pos(), |t| t.get_pos())
    }
}
