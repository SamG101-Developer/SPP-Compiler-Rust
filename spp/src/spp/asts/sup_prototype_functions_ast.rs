use crate::spp::analyse::scopes::scope::Scope;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::sup_implementation_ast::SupImplementationAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;

#[derive(Clone, Debug)]
pub struct SupPrototypeFunctionsAst {
    pos: usize,
    tok_sup: TokenAst,
    generic_param_group: Option<GenericParameterGroupAst>,
    name: TypeAst,
    where_block: Option<WhereBlockAst>,
    body: SupImplementationAst,
    scope_cls: Option<Scope>,
}

impl SupPrototypeFunctionsAst {
    pub fn new(
        pos: usize,
        tok_sup: TokenAst,
        generic_param_group: Option<GenericParameterGroupAst>,
        name: TypeAst,
        where_block: Option<WhereBlockAst>,
        body: SupImplementationAst,
    ) -> Self {
        Self {
            pos,
            tok_sup,
            generic_param_group,
            name,
            where_block,
            body,
            scope_cls: None,
        }
    }
}

impl Ast for SupPrototypeFunctionsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
