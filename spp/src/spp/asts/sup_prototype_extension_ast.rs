use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::{Ast, PreProcessingContext};
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::sup_implementation_ast::SupImplementationAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;

#[derive(Clone, Debug)]
pub struct SupPrototypeExtensionAst {
    pub pos: usize,
    pub tok_sup: TokenAst,
    pub generic_param_group: Option<GenericParameterGroupAst>,
    pub name: TypeAst,
    pub tok_ext: TokenAst,
    pub superclass: TypeAst,
    pub where_block: Option<WhereBlockAst>,
    pub body: SupImplementationAst,
    pub scope_cls: Option<Scope>,

    _ctx: Option<PreProcessingContext>,
}

impl SupPrototypeExtensionAst {
    pub fn new(
        pos: usize,
        tok_sup: TokenAst,
        generic_param_group: Option<GenericParameterGroupAst>,
        name: TypeAst,
        tok_ext: TokenAst,
        superclass: TypeAst,
        where_block: Option<WhereBlockAst>,
        body: SupImplementationAst,
    ) -> Self {
        Self {
            pos,
            tok_sup,
            generic_param_group,
            name,
            tok_ext,
            superclass,
            where_block,
            body,
            scope_cls: None,
            _ctx: None,
        }
    }
}

impl Ast for SupPrototypeExtensionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }

    fn stage_1_preprocess_asts(&mut self, context: PreProcessingContext) -> Result<(), SemanticError> {
        // if self.name.type_parts()[0].value[0] == "$" { return Ok(()); }
        // self._ctx = Some(context.clone());
        // self.body.stage_1_preprocess_asts(context)?;
        Ok(())
    }
}
