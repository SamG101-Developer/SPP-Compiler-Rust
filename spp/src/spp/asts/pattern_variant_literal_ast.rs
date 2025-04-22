use crate::spp::asts::ast::Ast;
use crate::spp::asts::literal_boolean_ast::LiteralBooleanAst;
use crate::spp::asts::literal_float_ast::LiteralFloatAst;
use crate::spp::asts::literal_integer_ast::LiteralIntegerAst;
use crate::spp::asts::literal_string_ast::LiteralStringAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantLiteralAst {
    Float(LiteralFloatAst),
    Integer(LiteralIntegerAst),
    String(LiteralStringAst),
    Boolean(LiteralBooleanAst),
}
