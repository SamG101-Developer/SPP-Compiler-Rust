use crate::spp::asts::ast::Ast;
use crate::spp::asts::generic_argument_comp_named_ast::GenericArgumentCompNamedAst;
use crate::spp::asts::generic_argument_comp_unnamed_ast::GenericArgumentCompUnnamedAst;
use crate::spp::asts::generic_argument_type_named_ast::GenericArgumentTypeNamedAst;
use crate::spp::asts::generic_argument_type_unnamed_ast::GenericArgumentTypeUnnamedAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum GenericArgumentAst {
    CompNamed(GenericArgumentCompNamedAst),
    CompUnnamed(GenericArgumentCompUnnamedAst),
    TypeNamed(GenericArgumentTypeNamedAst),
    TypeUnnamed(GenericArgumentTypeUnnamedAst),
}
