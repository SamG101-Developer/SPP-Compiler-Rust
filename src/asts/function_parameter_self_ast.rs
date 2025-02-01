pub struct FunctionParameterSelfAst {
    pub pos: usize,
    pub tok_mut: TokenAst,
    pub convention: TokenAst,
    pub name: TokenAst,
    _type: TypeAst,
}

impl FunctionParameterSelfAst {
    pub fn new(pos: usize, tok_mut: TokenAst, convention: TokenAst, name: TokenAst) -> Self {
        Self {
            pos,
            tok_mut,
            convention,
            name,
            _type: CommonTypes::Self_(),
        }
    }
}
