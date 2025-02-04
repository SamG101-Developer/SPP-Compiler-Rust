use crate::analyse::utilities::common_types::CommonTypes;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

pub struct TypeOptionalAst {
    pub pos: usize,
    pub tok_question: TokenAst,
    pub type_: TypeAst,
}

impl TypeOptionalAst {
    pub fn new(pos: usize, tok_question: TokenAst, type_: TypeAst) -> Self {
        Self {
            pos,
            tok_question,
            type_,
        }
    }

    pub fn to_type(self) -> TypeAst {
        CommonTypes::optional(self.type_, self.pos)
    }
}
