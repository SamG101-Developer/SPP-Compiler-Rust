pub struct GenericArgumentGroupAst {
    pos: usize,
    tok_bracket_l: TokenAst,
    args: Vec<GenericArgumentAst>,
    tok_bracket_r: TokenAst,
}

impl GenericArgumentGroupAst {
    pub fn new(
        pos: usize,
        tok_bracket_l: TokenAst,
        arguments: Vec<GenericArgumentAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_bracket_l,
            args: arguments,
            tok_bracket_r,
        }
    }
}
