use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::literal_boolean_ast::LiteralBooleanAst;
use crate::spp::asts::literal_float_ast::LiteralFloatAst;
use crate::spp::asts::literal_integer_ast::LiteralIntegerAst;
use crate::spp::asts::literal_string_ast::LiteralStringAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum PatternVariantLiteralAst {
    Float(LiteralFloatAst),
    Integer(LiteralIntegerAst),
    String(LiteralStringAst),
    Boolean(LiteralBooleanAst),
}
