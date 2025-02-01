pub struct FunctionRequiredParameterAst {
    pub pos: usize,
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
}

impl FunctionRequiredParameterAst {
    pub fn new(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            variable,
            tok_colon,
            type_,
        }
    }
}
