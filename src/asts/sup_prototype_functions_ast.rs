pub struct SupPrototypeFunctionsAst {
    pos: usize,
    tok_sup: TokenAst,
    generic_param_group: GenericParameterGroupAst,
    name: TypeAst,
    where_block: WhereBlockAst,
    body: SupImplementationAst,
    scope_cls: Option<Scope>,
}

impl SupPrototypeFunctionsAst {
    pub(crate) fn new(
        pos: usize,
        tok_sup: TokenAst,
        generic_param_group: GenericParameterGroupAst,
        name: TypeAst,
        where_block: WhereBlockAst,
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
