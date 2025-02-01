pub struct GenericParameterCompRequiredAst {
    pub pos: usize,
    pub tok_comp: TokenAst,
    pub name: TypeAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
}

impl GenericParameterCompRequiredAst {
    pub fn new(
        pos: usize,
        tok_comp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_comp,
            name,
            tok_colon,
            type_,
        }
    }
}
