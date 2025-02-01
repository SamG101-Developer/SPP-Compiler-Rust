pub struct FunctionCallArgumentUnnamedAst {
    pub pos: usize,
    pub convention: ConventionAst,
    pub tok_unpack: Option<TokenAst>,
    pub value: ExpressionAst,
}

impl FunctionCallArgumentUnnamedAst {
    pub fn new(
        pos: usize,
        convention: ConventionAst,
        tok_unpack: Option<TokenAst>,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            convention,
            tok_unpack,
            value,
        }
    }
}
