pub enum SupMemberAst {
    Method(FunctionProtoypeAst),
    Typedef(SupUseStatementAst),
    SupExtension(SupExtensionProtypeAst),
}
