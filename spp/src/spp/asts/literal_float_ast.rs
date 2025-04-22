use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LiteralFloatAst {
    pos: usize,
    tok_sign: Option<TokenAst>,
    integer_value: TokenAst,
    tok_dot: TokenAst,
    float_value: TokenAst,
    type_: Option<TypeAst>,
}

impl LiteralFloatAst {
    pub fn new(
        pos: usize,
        tok_sign: Option<TokenAst>,
        integer_value: TokenAst,
        tok_dot: TokenAst,
        float_value: TokenAst,
        type_: Option<TypeAst>,
    ) -> Self {
        Self {
            pos,
            tok_sign,
            integer_value,
            tok_dot,
            float_value,
            type_,
        }
    }
}

impl Ast for LiteralFloatAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        if let Some(type_) = &self.type_ {
            type_.get_final_pos()
        } else {
            self.float_value.get_final_pos()
        }
    }
}
