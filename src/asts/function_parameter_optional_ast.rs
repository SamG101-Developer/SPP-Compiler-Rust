pub struct FunctionParameterOptionalAst {
    pub pos: usize,
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub convention: ConventionAst,
    pub type_: TypeAst,
    pub tok_assign: TokenAst,
    pub value: ExpressionAst,
}

impl FunctionParameterOptionalAst {
    pub fn new(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    ) -> Self {
        Self {
            pos,
            variable,
            tok_colon,
            convention,
            type_,
            tok_assign,
            value,
        }
    }
}
