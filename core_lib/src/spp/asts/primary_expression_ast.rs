use crate::spp::asts::ast::Ast;
use crate::spp::asts::case_expression_ast::CaseExpressionAst;
use crate::spp::asts::gen_expression_ast::GenExpressionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::is_expression_ast::IsExpressionAst;
use crate::spp::asts::literal_ast::LiteralAst;
use crate::spp::asts::loop_expression_ast::LoopExpressionAst;
use crate::spp::asts::object_initializer::ObjectInitializerAst;
use crate::spp::asts::parenthesized_expression_ast::ParenthesizedExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::with_expression_ast::WithExpressionAst;

#[derive(Clone, Debug)]
pub enum PrimaryExpressionAst {
    Literal(LiteralAst),
    Identifier(IdentifierAst),
    SelfIdentifier(IdentifierAst),
    Parenthesized(ParenthesizedExpressionAst),
    Gen(GenExpressionAst),
    ObjectInitializer(ObjectInitializerAst),
    InnerScope(InnerScopeAst),
    Case(CaseExpressionAst),
    Loop(LoopExpressionAst),
    With(WithExpressionAst),
    Type(TypeAst),
    Fold(TokenAst),
    Is(IsExpressionAst),
}

impl Ast for PrimaryExpressionAst {
    fn get_pos(&self) -> usize {
        match self {
            PrimaryExpressionAst::Literal(literal) => literal.get_pos(),
            PrimaryExpressionAst::Identifier(identifier) => identifier.get_pos(),
            PrimaryExpressionAst::SelfIdentifier(token) => token.get_pos(),
            PrimaryExpressionAst::Parenthesized(parenthesized) => parenthesized.get_pos(),
            PrimaryExpressionAst::Gen(gen) => gen.get_pos(),
            PrimaryExpressionAst::ObjectInitializer(object_initializer) => { object_initializer.get_pos() }
            PrimaryExpressionAst::InnerScope(inner_scope) => inner_scope.get_pos(),
            PrimaryExpressionAst::Case(case) => case.get_pos(),
            PrimaryExpressionAst::Loop(loop_) => loop_.get_pos(),
            PrimaryExpressionAst::With(with) => with.get_pos(),
            PrimaryExpressionAst::Type(type_) => type_.get_pos(),
            PrimaryExpressionAst::Fold(token) => token.get_pos(),
            PrimaryExpressionAst::Is(is) => is.get_pos(),
        }
    }
}
