use crate::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::asts::sup_use_statement_ast::SupUseStatementAst;

pub enum SupMemberAst {
    Method(FunctionPrototypeAst),
    Typedef(SupUseStatementAst),
    SupExtension(SupPrototypeExtensionAst),
}
