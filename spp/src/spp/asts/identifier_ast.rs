use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug, Default)]
pub struct IdentifierAst {
    pub pos: usize,
    pub value: String,
}

impl IdentifierAst {
    pub fn new(pos: usize, value: String) -> Self {
        Self { pos, value }
    }
}

impl Ast for IdentifierAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.pos + self.value.len()
    }
}

impl From<&TypeAst> for IdentifierAst {
    fn from(type_: &TypeAst) -> Self {
        match type_ {
            TypeAst::Single(type_) => IdentifierAst::new(type_.pos, type_.name.clone().value),
            _ => panic!("TypeAst::Single expected"),
        }
    }
}

impl From<&GenericIdentifierAst> for IdentifierAst {
    fn from(value: &GenericIdentifierAst) -> Self {
        IdentifierAst::new(value.pos, value.value.clone())
    }
}
