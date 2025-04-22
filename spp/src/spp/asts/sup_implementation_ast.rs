use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::sup_member_ast::SupMemberAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct SupImplementationAst {
    pub pos: usize,
    pub tok_brace_l: TokenAst,
    pub members: Vec<SupMemberAst>,
    pub tok_brace_r: TokenAst,
}

impl SupImplementationAst {
    pub fn new(
        pos: usize,
        tok_brace_l: TokenAst,
        members: Vec<SupMemberAst>,
        tok_brace_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_brace_l,
            members,
            tok_brace_r,
        }
    }
}

impl Ast for SupImplementationAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_brace_r.get_final_pos()
    }

    fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
        self.members.iter_mut().try_for_each(|m| m.stage_1_preprocess_asts(context.clone()))?;
        Ok(())
    }
}
