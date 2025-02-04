use crate::asts::ast::Ast;
use crate::asts::function_prototype_ast::FunctionPrototypeAst;
use crate::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::asts::sup_use_statement_ast::SupUseStatementAst;

#[derive(Clone)]
pub enum SupMemberAst {
    Method(FunctionPrototypeAst),
    Typedef(SupUseStatementAst),
    SupExtension(SupPrototypeExtensionAst),
}

impl Ast for SupMemberAst {
    fn get_pos(&self) -> usize {
        match self {
            SupMemberAst::Method(method) => method.get_pos(),
            SupMemberAst::Typedef(typedef) => typedef.get_pos(),
            SupMemberAst::SupExtension(extension) => extension.get_pos(),
        }
    }
}
