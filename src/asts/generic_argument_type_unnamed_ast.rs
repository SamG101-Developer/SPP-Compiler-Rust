pub struct GenericTypeArgumentUnnamedAst {
    pub pos: usize,
    pub type_: TypeAst,
}

impl GenericTypeArgumentUnnamedAst {
    pub fn new(pos: usize, type_: TypeAst) -> Self {
        Self { pos, type_ }
    }
}
