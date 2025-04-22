use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::case_expression_ast::CaseExpressionAst;
use crate::spp::asts::gen_expression_ast::GenExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::literal_ast::LiteralAst;
use crate::spp::asts::loop_expression_ast::LoopExpressionAst;
use crate::spp::asts::object_initializer::ObjectInitializerAst;
use crate::spp::asts::parenthesized_expression_ast::ParenthesizedExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use enum_dispatch::enum_dispatch;

#[derive(Clone, Debug)]
#[enum_dispatch(Ast)]
pub enum PrimaryExpressionAst {
    Literal(LiteralAst),
    Identifier(IdentifierAst),
    Parenthesized(ParenthesizedExpressionAst),
    Gen(GenExpressionAst),
    ObjectInitializer(ObjectInitializerAst),
    InnerScope(InnerScopeAst),
    Case(CaseExpressionAst),
    Loop(LoopExpressionAst),
    Type(TypeAst),
    Fold(TokenAst),
}
