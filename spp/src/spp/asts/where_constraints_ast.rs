use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct WhereConstraintsAst {
    pub types: Vec<TypeAst>,
    pub tok_colon: TokenAst,
    pub constraints: TypeAst,
}

impl WhereConstraintsAst {
    pub fn new(types: Vec<TypeAst>, tok_colon: TokenAst, constraints: TypeAst) -> Self {
        Self { types, tok_colon, constraints }
    }
}

impl Ast for WhereConstraintsAst {
    fn get_pos(&self) -> usize {
        self.types.first().as_ref().unwrap().get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.constraints.get_final_pos()
    }
}
