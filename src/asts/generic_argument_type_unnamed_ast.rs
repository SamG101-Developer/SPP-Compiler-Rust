pub struct GenericArgumentTypeUnnamedAst {
    pub pos: usize,
    pub type_: TypeAst,
}

impl GenericArgumentTypeUnnamedAst {
    pub fn new(pos: usize, type_: TypeAst) -> Self {
        Self { pos, type_ }
    }
}
