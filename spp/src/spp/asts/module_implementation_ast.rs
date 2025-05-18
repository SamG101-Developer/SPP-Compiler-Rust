use crate::spp::asts::ast::Ast;
use crate::spp::asts::module_member_ast::ModuleMemberAst;

#[derive(Clone, Debug, Default)]
pub struct ModuleImplementationAst {
    pub members: Vec<ModuleMemberAst>,
}

impl ModuleImplementationAst {
    pub fn new(members: Vec<ModuleMemberAst>) -> Self {
        Self { members }
    }
}

impl Ast for ModuleImplementationAst {
    fn get_pos(&self) -> usize {
        self.members.last().as_ref().map_or(0, |m| m.get_pos())
    }
    
    fn get_final_pos(&self) -> usize {
        self.members.last().unwrap().get_final_pos()
    }

    // fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
    //     self.members.iter_mut().try_for_each(|m| m.stage_1_preprocess_asts(context.clone()))?;
    //     Ok(())
    // }
}
