pub enum ModuleMemberAst {
    Class(ClassPrototypeAst),
    Function(FunctionPrototypeAst),
    SupExtension(SupPrototypeExtensionAst),
    SupFunctions(SupPrototypeFunctionsAst),
    UseStatement(UseStatementAst),
    GlobalConst(GlobalConstAst),
}
