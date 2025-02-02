use crate::asts::pattern_variant_attribute_binding_ast::PatternVariantAttributeBindingAst;
use crate::asts::pattern_variant_destructure_array_ast::PatternVariantDestructureArrayAst;
use crate::asts::pattern_variant_destructure_object_ast::PatternVariantDestructureObjectAst;
use crate::asts::pattern_variant_destructure_skip_1_argument_ast::PatternVariantDestructureSkip1ArgumentAst;
use crate::asts::pattern_variant_destructure_skip_n_args_ast::PatternVariantDestructureSkipNArgumentsAst;
use crate::asts::pattern_variant_destructure_tuple_ast::PatternVariantDestructureTupleAst;
use crate::asts::pattern_variant_else_ast::PatternVariantElseAst;
use crate::asts::pattern_variant_else_case_ast::PatternVariantElseCaseAst;
use crate::asts::pattern_variant_expression_ast::PatternVariantExpressionAst;
use crate::asts::pattern_variant_literal_ast::PatternVariantLiteralAst;
use crate::asts::pattern_variant_single_identifier_ast::PatternVariantSingleIdentifierAst;

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

pub enum PatternVariantNestedForDestructureObjectAst {
    AttrBind(PatternVariantAttributeBindingAst),
    SkipNArgs(PatternVariantDestructureSkipNArgumentsAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

pub enum PatternVariantNestedForAttributeBindingAst {
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    Literal(PatternVariantLiteralAst),
}
