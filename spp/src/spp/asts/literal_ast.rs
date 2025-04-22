use crate::spp::asts::ast::Ast;
use crate::spp::asts::literal_array_ast::LiteralArrayAst;
use crate::spp::asts::literal_array_empty_ast::LiteralArrayEmptyAst;
use crate::spp::asts::literal_boolean_ast::LiteralBooleanAst;
use crate::spp::asts::literal_float_ast::LiteralFloatAst;
use crate::spp::asts::literal_integer_ast::LiteralIntegerAst;
use crate::spp::asts::literal_string_ast::LiteralStringAst;
use crate::spp::asts::literal_tuple_ast::LiteralTupleAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum LiteralAst {
    Boolean(LiteralBooleanAst),
    Integer(LiteralIntegerAst),
    Float(LiteralFloatAst),
    String(LiteralStringAst),
    Array0(LiteralArrayEmptyAst),
    ArrayN(LiteralArrayAst),
    Tuple(LiteralTupleAst),
}
