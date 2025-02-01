pub struct FunctionParameterVariadicAst {
    pub pos: usize,
    pub tok_variadic: TokenAst,
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub convention: ConventionAst,
    pub type_: TypeAst,
}

impl FunctionParameterVariadicAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            variable,
            tok_colon,
            convention,
            type_,
        }
    }
}
