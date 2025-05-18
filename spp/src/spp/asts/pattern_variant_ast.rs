use crate::spp::asts::ast::Ast;
use crate::spp::asts::ast::ToBinaryExpression;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::is_expression_ast::IsExpressionAst;
use crate::spp::asts::pattern_variant_attribute_binding_ast::PatternVariantAttributeBindingAst;
use crate::spp::asts::pattern_variant_destructure_array_ast::PatternVariantDestructureArrayAst;
use crate::spp::asts::pattern_variant_destructure_object_ast::PatternVariantDestructureObjectAst;
use crate::spp::asts::pattern_variant_destructure_skip_1_argument_ast::PatternVariantDestructureSkip1ArgumentAst;
use crate::spp::asts::pattern_variant_destructure_skip_n_args_ast::PatternVariantDestructureSkipNArgumentsAst;
use crate::spp::asts::pattern_variant_destructure_tuple_ast::PatternVariantDestructureTupleAst;
use crate::spp::asts::pattern_variant_else_ast::PatternVariantElseAst;
use crate::spp::asts::pattern_variant_else_case_ast::PatternVariantElseCaseAst;
use crate::spp::asts::pattern_variant_expression_ast::PatternVariantExpressionAst;
use crate::spp::asts::pattern_variant_literal_ast::PatternVariantLiteralAst;
use crate::spp::asts::pattern_variant_single_identifier_ast::PatternVariantSingleIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantAst {
    Else(PatternVariantElseAst),
    ElseCase(PatternVariantElseCaseAst),
    Expression(PatternVariantExpressionAst),
    Literal(PatternVariantLiteralAst),
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantNestedForDestructureArrayAst {
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    Skip1Args(PatternVariantDestructureSkip1ArgumentAst),
    SkipNArgs(PatternVariantDestructureSkipNArgumentsAst),
    Expression(PatternVariantExpressionAst),
    Literal(PatternVariantLiteralAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantNestedForDestructureTupleAst {
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    Skip1Args(PatternVariantDestructureSkip1ArgumentAst),
    SkipNArgs(PatternVariantDestructureSkipNArgumentsAst),
    Expression(PatternVariantExpressionAst),
    Literal(PatternVariantLiteralAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantNestedForDestructureObjectAst {
    AttrBind(PatternVariantAttributeBindingAst),
    SkipNArgs(PatternVariantDestructureSkipNArgumentsAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

#[derive(Clone, Debug)]
#[delegation::delegate(derive(Ast))]
pub enum PatternVariantNestedForAttributeBindingAst {
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    Literal(PatternVariantLiteralAst),
}

impl ToBinaryExpression for PatternVariantAst {
    fn to_binary_expression(lhs: ExpressionAst, op: TokenAst, rhs: Self) -> ExpressionAst {
        ExpressionAst::Is(IsExpressionAst::new(Box::new(lhs), op, Box::new(rhs)))
    }
}
