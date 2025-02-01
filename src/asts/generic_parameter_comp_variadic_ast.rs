pub struct GenericParameterCompVariadicAst {
    pub pos: usize,
    pub tok_cmp: TokenAst,
    pub tok_variadic: TokenAst,
    pub name: TypeAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
}

impl GenericParameterCompVariadicAst {
    pub fn new(
        pos: usize,
        tok_cmp: TokenAst,
        tok_variadic: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_cmp,
            tok_variadic,
            name,
            tok_colon,
            type_,
        }
    }
}
