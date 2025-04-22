use crate::spp::analyse::scopes::scope_manager::ScopeManager;
use crate::spp::analyse::utilities::semantic_error::SemanticError;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::PreProcessingContext;
use crate::spp::asts::ast::ToBinaryExpression;
use crate::spp::asts::binary_expression_ast::BinaryExpressionAst;
use crate::spp::asts::is_expression_ast::IsExpressionAst;
use crate::spp::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::spp::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::unary_expression_ast::UnaryExpressionAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum ExpressionAst {
    Binary(BinaryExpressionAst),
    Is(IsExpressionAst),
    Postfix(PostfixExpressionAst),
    Unary(UnaryExpressionAst),
    Primary(PrimaryExpressionAst),
}


impl ToBinaryExpression for ExpressionAst {
    fn to_binary_expression(
        pos: usize,
        lhs: ExpressionAst,
        op: TokenAst,
        rhs: Self,
    ) -> ExpressionAst {
        ExpressionAst::Binary(BinaryExpressionAst::new(
            pos,
            Box::new(lhs),
            op,
            Box::new(rhs),
        ))
    }
}
