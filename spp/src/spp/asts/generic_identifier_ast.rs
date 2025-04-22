use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::spp::asts::identifier_ast::IdentifierAst;

#[derive(Clone, Debug)]
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

impl From<IdentifierAst> for GenericIdentifierAst {
    fn from(identifier: IdentifierAst) -> Self {
        Self {
            pos: identifier.pos,
            value: identifier.value.clone(),
            generic_args_group: None,
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

    fn get_final_pos(&self) -> usize {
        if let Some(generic_args_group) = &self.generic_args_group {
            generic_args_group.get_final_pos()
        } else {
            self.pos + self.value.len()
        }
    }
}
