pub enum FunctionParameterAst {
    Self_(FunctionSelfParameterAst),
    Required(FunctionRequiredParameterAst),
    Optional(FunctionOptionalParameterAst),
    Variadic(FunctionVariadicParameterAst),
}
