use crate::asts::ast::Ast;
use crate::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone)]
pub struct GenericIdentifierAst {
    pub pos: usize,
    pub value: String,
    pub generic_args_group: Option<GenericArgumentGroupAst>,
}

impl GenericIdentifierAst {
    pub fn new(
        pos: usize,
        value: String,
        generic_args_group: Option<GenericArgumentGroupAst>,
    ) -> Self {
        Self {
            pos,
            value,
            generic_args_group,
        }
    }
}

impl From<&TypeAst> for GenericIdentifierAst {
    fn from(type_: &TypeAst) -> Self {
        GenericIdentifierAst {
            pos: type_.get_pos(),
            value: type_.types.last().unwrap().value.clone(),
            generic_args_group: type_.types.last().unwrap().generic_args_group.clone(),
        }
    }
}

impl From<&IdentifierAst> for GenericIdentifierAst {
    fn from(identifier: &IdentifierAst) -> Self {
        Self {
            pos: identifier.pos,
            value: identifier.value.clone(),
            generic_args_group: None,
        }
    }
}

impl Ast for GenericIdentifierAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
