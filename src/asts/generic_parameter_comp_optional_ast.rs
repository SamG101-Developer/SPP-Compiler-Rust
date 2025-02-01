pub struct GenericParameterCompOptionalAst {
    pub pos: usize,
    pub tok_cmp: TokenAst,
    pub name: TypeAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    pub tok_assign: TokenAst,
    pub default: TypeAst,
}

impl GenericParameterCompOptionalAst {
    pub fn new(
        pos: usize,
        tok_cmp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        default: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_cmp,
            name,
            tok_colon,
            type_,
            tok_assign,
            default,
        }
    }
}
