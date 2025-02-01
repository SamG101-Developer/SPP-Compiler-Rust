pub struct GenericParameterTypeInlineConstraintsAst {
    pub pos: usize,
    pub tok_colon: TokenAst,
    pub constraints: Vec<TypeAst>,
}

impl GenericParameterTypeInlineConstraintsAst {
    pub fn new(pos: usize, tok_colon: TokenAst, constraints: Vec<TypeAst>) -> Self {
        Self {
            pos,
            tok_colon,
            constraints,
        }
    }
}
