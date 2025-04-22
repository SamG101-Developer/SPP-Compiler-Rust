use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_mut_ast::ConventionMutAst;
use crate::spp::asts::convention_ref_ast::ConventionRefAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum ConventionAst {
    Mut(ConventionMutAst),
    Ref(ConventionRefAst),
}
