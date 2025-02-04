use crate::analyse::scopes::scope::Scope;
use crate::asts::ast::Ast;
use crate::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::asts::sup_implementation_ast::SupImplementationAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;
use crate::asts::where_block_ast::WhereBlockAst;

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
}
