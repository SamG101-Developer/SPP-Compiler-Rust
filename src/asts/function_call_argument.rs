pub enum FunctionCallArgumentAst {
    NamedArgument(FunctionCallNamedArgumentAst),
    UnnamedArgument(FunctionCallUnnamedArgumentAst),
}
