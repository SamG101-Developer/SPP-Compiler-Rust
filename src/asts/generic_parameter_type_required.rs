pub struct GenericParameterTypeRequiredAst {
    pub pos: usize,
    pub name: TypeAst,
    pub constraints: Vec<GenericParameterTypeInlineConstraints>,
}

impl GenericParameterTypeRequiredAst {
    pub fn new(
        pos: usize,
        name: TypeAst,
        constraints: Vec<GenericParameterTypeInlineConstraints>,
    ) -> Self {
        Self {
            pos,
            name,
            constraints,
        }
    }
}
