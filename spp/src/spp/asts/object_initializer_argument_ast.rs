use crate::spp::asts::ast::Ast;
use crate::spp::asts::object_initializer_argument_named::ObjectInitializerArgumentNamedAst;
use crate::spp::asts::object_initializer_argument_unnamed::ObjectInitializerArgumentUnnamedAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum ObjectInitializerArgumentAst {
    Named(ObjectInitializerArgumentNamedAst),
    Unnamed(ObjectInitializerArgumentUnnamedAst),
}
