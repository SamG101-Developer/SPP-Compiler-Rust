use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LiteralArrayEmptyAst {
    tok_bracket_l: TokenAst,
    elem_type: TypeAst,
    tok_comma: TokenAst,
    size: TokenAst,
    tok_bracket_r: TokenAst,
}

impl LiteralArrayEmptyAst {
    pub fn new(tok_bracket_l: TokenAst, elem_type: TypeAst, tok_comma: TokenAst, size: TokenAst, tok_bracket_r: TokenAst) -> Self {
        Self { tok_bracket_l, elem_type, tok_comma, size, tok_bracket_r }
    }
}

impl Ast for LiteralArrayEmptyAst {
    fn get_pos(&self) -> usize {
        self.tok_bracket_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_bracket_r.get_final_pos()
    }
}
