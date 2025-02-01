pub struct FunctionCallArgumentNamedAst {
    pub pos: usize,
    pub name: IdentifierAst,
    pub tok_assign: TokenAst,
    pub convention: ConventionAst,
    pub value: ExpressionAst,
}

impl FunctionCallArgumentNamedAst {
    pub fn new(
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        convention: ConventionAst,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            convention,
            value,
        }
    }
}
