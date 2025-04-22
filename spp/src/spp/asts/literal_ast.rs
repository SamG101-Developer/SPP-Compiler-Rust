use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::literal_array_ast::LiteralArrayAst;
use crate::spp::asts::literal_array_empty_ast::LiteralArrayEmptyAst;
use crate::spp::asts::literal_boolean_ast::LiteralBooleanAst;
use crate::spp::asts::literal_float_ast::LiteralFloatAst;
use crate::spp::asts::literal_integer_ast::LiteralIntegerAst;
use crate::spp::asts::literal_string_ast::LiteralStringAst;
use crate::spp::asts::literal_tuple_ast::LiteralTupleAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum LiteralAst {
    Boolean(LiteralBooleanAst),
    Integer(LiteralIntegerAst),
    Float(LiteralFloatAst),
    String(LiteralStringAst),
    Array0(LiteralArrayEmptyAst),
    ArrayN(LiteralArrayAst),
    Tuple(LiteralTupleAst),
}
