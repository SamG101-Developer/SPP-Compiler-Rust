use crate::spp::asts::ast::Ast;
use crate::spp::asts::function_call_argument_named_ast::FunctionCallArgumentNamedAst;
use crate::spp::asts::function_call_argument_unnamed_ast::FunctionCallArgumentUnnamedAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum FunctionCallArgumentAst {
    Named(FunctionCallArgumentNamedAst),
    Unnamed(FunctionCallArgumentUnnamedAst),
}
