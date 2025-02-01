pub struct GenericArgumentCompNamedAst {
    pub pos: usize,
    pub name: TypeAst,
    pub tok_assign: TokenAst,
    pub value: ExpressionAst,
}

impl GenericArgumentCompNamedAst {
    pub fn new(pos: usize, name: TypeAst, tok_assign: TokenAst, value: ExpressionAst) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            value,
        }
    }
}
