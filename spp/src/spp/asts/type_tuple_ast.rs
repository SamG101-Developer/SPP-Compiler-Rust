use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

pub struct TypeTupleAst {
    pub pos: usize,
    pub tok_parenthesis_left: TokenAst,
    pub elements: Vec<TypeAst>,
    pub tok_parenthesis_right: TokenAst,
}

impl TypeTupleAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_left: TokenAst,
        elements: Vec<TypeAst>,
        tok_parenthesis_right: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_left,
            elements,
            tok_parenthesis_right,
        }
    }

    pub fn to_type(self) -> TypeAst {
        CommonTypes::tuple(self.elements, self.pos)
    }
}
