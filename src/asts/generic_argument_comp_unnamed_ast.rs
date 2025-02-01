pub struct GenericArgumentCompUnnamedAst {
    pub pos: usize,
    pub value: ExpressionAst,
}

impl GenericArgumentCompUnnamedAst {
    pub fn new(pos: usize, value: ExpressionAst) -> Self {
        Self { pos, value }
    }
}
