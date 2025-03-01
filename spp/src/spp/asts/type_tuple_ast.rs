use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

pub struct TypeTupleAst {
    pub pos: usize,
    pub tok_left_parenthesis: TokenAst,
    pub type_list: Vec<TypeAst>,
    pub tok_right_parenthesis: TokenAst,
}

impl TypeTupleAst {
    pub fn new(
        pos: usize,
        tok_left_parenthesis: TokenAst,
        type_list: Vec<TypeAst>,
        tok_right_parenthesis: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_left_parenthesis,
            type_list,
            tok_right_parenthesis,
        }
    }
}

impl Ast for TypeTupleAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_right_parenthesis.get_final_pos()
    }
}
