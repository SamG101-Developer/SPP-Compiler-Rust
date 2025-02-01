pub struct GenericParameterTypeOptionalAst {
    pub pos: usize,
    pub name: TypeAst,
    pub constraints: Vec<GenericParameterTypeInlineConstraints>,
    pub tok_assign: TokenAst,
    pub default: TypeAst,
}

impl GenericParameterTypeOptionalAst {
    pub fn new(
        pos: usize,
        name: TypeAst,
        constraints: Vec<GenericParameterTypeInlineConstraints>,
        tok_assign: TokenAst,
        default: TypeAst,
    ) -> Self {
        Self {
            pos,
            name,
            constraints,
            tok_assign,
            default,
        }
    }
}
