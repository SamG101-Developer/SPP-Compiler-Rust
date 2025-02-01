pub struct GenericCompArgumentUnnamedAst {
    pub pos: usize,
    pub value: ExpressionAst,
}

impl GenericCompArgumentUnnamedAst {
    pub fn new(pos: usize, value: ExpressionAst) -> Self {
        Self { pos, value }
    }
}
