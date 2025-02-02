pub struct TokenAst {
    pub pos: usize,
    pub metadata: String,
}

impl TokenAst {
    pub fn new(pos: usize, metadata: String) -> Self {
        Self { pos, metadata }
    }
}

impl Default for TokenAst {
    fn default() -> Self {
        Self {
            pos: 0,
            metadata: String::new(),
        }
    }
}
