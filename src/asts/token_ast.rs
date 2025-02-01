pub struct TokenAst {
    pub pos: usize,
    pub metadata: String,
}

impl TokenAst {
    pub fn new(pos: usize, metadata: String) -> Self {
        Self { pos, metadata }
    }
}
