pub struct FunctionParameterRequiredAst {
    pub pos: usize,
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub convention: ConventionAst,
    pub type_: TypeAst,
}

impl FunctionParameterRequiredAst {
    pub fn new(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            variable,
            tok_colon,
            convention,
            type_,
        }
    }
}
