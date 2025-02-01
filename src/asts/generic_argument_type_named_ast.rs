pub struct GenericArgumentTypeNamedAst {
    pub pos: usize,
    pub name: TypeAst,
    pub tok_assign: TokenAst,
    pub value: TypeAst,
}

impl GenericArgumentTypeNamedAst {
    pub fn new(pos: usize, name: TypeAst, tok_assign: TokenAst, value: TypeAst) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            value,
        }
    }
}
