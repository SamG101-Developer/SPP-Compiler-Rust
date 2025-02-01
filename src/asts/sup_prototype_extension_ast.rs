pub struct SupPrototypeExtensionAst {
    pos: usize,
    tok_sup: TokenAst,
    generic_param_group: GenericParameterGroupAst,
    name: TypeAst,
    tok_ext: TokenAst,
    superclass: TypeAst,
    where_block: WhereBlockAst,
    body: SupImplementationAst,
    scope_cls: Option<Scope>,
}

impl SupPrototypeExtensionAst {
    pub(crate) fn new(
        pos: usize,
        tok_sup: TokenAst,
        generic_param_group: GenericParameterGroupAst,
        name: TypeAst,
        tok_ext: TokenAst,
        superclass: TypeAst,
        where_block: WhereBlockAst,
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
        }
    }
}
