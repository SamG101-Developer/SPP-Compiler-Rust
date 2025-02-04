use crate::asts::ast::Ast;
use crate::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
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
}

impl From<&TypeAst> for IdentifierAst {
    fn from(type_: &TypeAst) -> Self {
        IdentifierAst::from(type_.types.last().unwrap())
    }
}

impl From<&GenericIdentifierAst> for IdentifierAst {
    fn from(value: &GenericIdentifierAst) -> Self {
        IdentifierAst::new(value.pos, value.value.clone())
    }
}
