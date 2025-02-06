use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::type_ast::TypeAst;

pub struct TypeVariantAst {
    pub pos: usize,
    pub elements: Vec<TypeAst>,
}

impl TypeVariantAst {
    pub fn new(pos: usize, elements: Vec<TypeAst>) -> Self {
        Self { pos, elements }
    }

    pub fn to_type(self) -> TypeAst {
        CommonTypes::variant(self.elements, self.pos)
    }
}
