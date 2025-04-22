use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::module_member_ast::ModuleMemberAst;

#[derive(Clone, Debug, Default)]
pub struct ModuleImplementationAst {
    pos: usize,
    members: Vec<ModuleMemberAst>,
}

impl ModuleImplementationAst {
    pub fn new(pos: usize, members: Vec<ModuleMemberAst>) -> Self {
        Self { pos, members }
    }
}

impl Ast for ModuleImplementationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.members.last().unwrap().get_final_pos()
    }

    fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
        self.members.iter_mut().try_for_each(|m| m.stage_1_preprocess_asts(context.clone()))?;
        Ok(())
    }
}
