pub struct FunctionParameterGroupAst {
    pub pos: usize,
    pub tok_paren_l: TokenAst,
    pub parameters: Vec<FunctionParameterAst>,
    pub tok_paren_r: TokenAst,
}

impl FunctionParameterGroupAst {
    pub fn new(
        pos: usize,
        tok_paren_l: TokenAst,
        parameters: Vec<FunctionParameterAst>,
        tok_paren_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_paren_l,
            parameters,
            tok_paren_r,
        }
    }
}
