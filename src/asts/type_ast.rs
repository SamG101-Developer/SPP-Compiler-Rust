use crate::asts::ast::Ast;
use crate::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::asts::identifier_ast::IdentifierAst;

#[derive(Clone)]
pub struct TypeAst {
    pub pos: usize,
    pub namespace: Vec<IdentifierAst>,
    pub types: Vec<GenericIdentifierAst>,
}

impl TypeAst {
    pub fn new(
        pos: usize,
        namespace: Vec<IdentifierAst>,
        types: Vec<GenericIdentifierAst>,
    ) -> Self {
        Self {
            pos,
            namespace,
            types,
        }
    }
}

impl Ast for TypeAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}

impl Default for TypeAst {
    fn default() -> Self {
        Self {
            pos: 0,
            namespace: vec![],
            types: vec![],
        }
    }
}

impl From<IdentifierAst> for TypeAst {
    fn from(identifier: IdentifierAst) -> Self {
        Self {
            pos: identifier.pos,
            namespace: vec![],
            types: vec![GenericIdentifierAst::from(&identifier)],
        }
    }
}

impl From<GenericIdentifierAst> for TypeAst {
    fn from(generic_identifier: GenericIdentifierAst) -> Self {
        Self {
            pos: generic_identifier.pos,
            namespace: vec![],
            types: vec![generic_identifier],
        }
    }
}
