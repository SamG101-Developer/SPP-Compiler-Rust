pub struct GenericParameterTypeVariadicAst {
    pub pos: usize,
    pub tok_variadic: TokenAst,
    pub name: TypeAst,
    pub constraints: Vec<GenericParameterTypeInlineConstraints>,
}

impl GenericParameterTypeVariadicAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        name: TypeAst,
        constraints: Vec<GenericParameterTypeInlineConstraints>,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            name,
            constraints,
        }
    }
}
