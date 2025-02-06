use crate::spp::asts::ast::{Ast, ToBinaryExpression};
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
use crate::spp::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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
pub enum PatternVariantNestedForDestructureObjectAst {
    AttrBind(PatternVariantAttributeBindingAst),
    SkipNArgs(PatternVariantDestructureSkipNArgumentsAst),
    SingleIdentifier(PatternVariantSingleIdentifierAst),
}

#[derive(Clone, Debug)]
pub enum PatternVariantNestedForAttributeBindingAst {
    DestructureArray(PatternVariantDestructureArrayAst),
    DestructureTuple(PatternVariantDestructureTupleAst),
    DestructureObject(PatternVariantDestructureObjectAst),
    Literal(PatternVariantLiteralAst),
}

impl Ast for PatternVariantAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantAst::Else(else_) => else_.get_pos(),
            PatternVariantAst::ElseCase(else_case) => else_case.get_pos(),
            PatternVariantAst::Expression(expression) => expression.get_pos(),
            PatternVariantAst::Literal(literal) => literal.get_pos(),
            PatternVariantAst::DestructureArray(destructure_array) => destructure_array.get_pos(),
            PatternVariantAst::DestructureTuple(destructure_tuple) => destructure_tuple.get_pos(),
            PatternVariantAst::DestructureObject(destructure_object) => { destructure_object.get_pos() }
            PatternVariantAst::SingleIdentifier(single_identifier) => single_identifier.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PatternVariantAst::Else(else_) => else_.get_final_pos(),
            PatternVariantAst::ElseCase(else_case) => else_case.get_final_pos(),
            PatternVariantAst::Expression(expression) => expression.get_final_pos(),
            PatternVariantAst::Literal(literal) => literal.get_final_pos(),
            PatternVariantAst::DestructureArray(destructure_array) => destructure_array.get_final_pos(),
            PatternVariantAst::DestructureTuple(destructure_tuple) => destructure_tuple.get_final_pos(),
            PatternVariantAst::DestructureObject(destructure_object) => destructure_object.get_final_pos(),
            PatternVariantAst::SingleIdentifier(single_identifier) => single_identifier.get_final_pos(),
        }
    }
}

impl Ast for PatternVariantNestedForDestructureArrayAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureArrayAst::DestructureArray(destructure_array) => { destructure_array.get_pos() }
            PatternVariantNestedForDestructureArrayAst::DestructureTuple(destructure_tuple) => { destructure_tuple.get_pos() }
            PatternVariantNestedForDestructureArrayAst::DestructureObject(destructure_object) => { destructure_object.get_pos() }
            PatternVariantNestedForDestructureArrayAst::Skip1Args(skip_1_args) => skip_1_args.get_pos(),
            PatternVariantNestedForDestructureArrayAst::SkipNArgs(skip_n_args) => skip_n_args.get_pos(),
            PatternVariantNestedForDestructureArrayAst::Expression(expression) => { expression.get_pos() }
            PatternVariantNestedForDestructureArrayAst::Literal(literal) => literal.get_pos(),
            PatternVariantNestedForDestructureArrayAst::SingleIdentifier(single_identifier) => { single_identifier.get_pos() }
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureArrayAst::DestructureArray(destructure_array) => { destructure_array.get_final_pos() }
            PatternVariantNestedForDestructureArrayAst::DestructureTuple(destructure_tuple) => { destructure_tuple.get_final_pos() }
            PatternVariantNestedForDestructureArrayAst::DestructureObject(destructure_object) => { destructure_object.get_final_pos() }
            PatternVariantNestedForDestructureArrayAst::Skip1Args(skip_1_args) => skip_1_args.get_final_pos(),
            PatternVariantNestedForDestructureArrayAst::SkipNArgs(skip_n_args) => skip_n_args.get_final_pos(),
            PatternVariantNestedForDestructureArrayAst::Expression(expression) => { expression.get_final_pos() }
            PatternVariantNestedForDestructureArrayAst::Literal(literal) => literal.get_final_pos(),
            PatternVariantNestedForDestructureArrayAst::SingleIdentifier(single_identifier) => { single_identifier.get_final_pos() }
        }
    }
}

impl Ast for PatternVariantNestedForDestructureTupleAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureTupleAst::DestructureArray(destructure_array) => { destructure_array.get_pos() }
            PatternVariantNestedForDestructureTupleAst::DestructureTuple(destructure_tuple) => { destructure_tuple.get_pos() }
            PatternVariantNestedForDestructureTupleAst::DestructureObject(destructure_object) => { destructure_object.get_pos() }
            PatternVariantNestedForDestructureTupleAst::Skip1Args(skip_1_args) => skip_1_args.get_pos(),
            PatternVariantNestedForDestructureTupleAst::SkipNArgs(skip_n_args) => skip_n_args.get_pos(),
            PatternVariantNestedForDestructureTupleAst::Expression(expression) => { expression.get_pos() }
            PatternVariantNestedForDestructureTupleAst::Literal(literal) => literal.get_pos(),
            PatternVariantNestedForDestructureTupleAst::SingleIdentifier(single_identifier) => { single_identifier.get_pos() }
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureTupleAst::DestructureArray(destructure_array) => { destructure_array.get_final_pos() }
            PatternVariantNestedForDestructureTupleAst::DestructureTuple(destructure_tuple) => { destructure_tuple.get_final_pos() }
            PatternVariantNestedForDestructureTupleAst::DestructureObject(destructure_object) => { destructure_object.get_final_pos() }
            PatternVariantNestedForDestructureTupleAst::Skip1Args(skip_1_args) => skip_1_args.get_final_pos(),
            PatternVariantNestedForDestructureTupleAst::SkipNArgs(skip_n_args) => skip_n_args.get_final_pos(),
            PatternVariantNestedForDestructureTupleAst::Expression(expression) => { expression.get_final_pos() }
            PatternVariantNestedForDestructureTupleAst::Literal(literal) => literal.get_final_pos(),
            PatternVariantNestedForDestructureTupleAst::SingleIdentifier(single_identifier) => { single_identifier.get_final_pos() }
        }
    }
}

impl Ast for PatternVariantNestedForDestructureObjectAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureObjectAst::AttrBind(attr_bind) => attr_bind.get_pos(),
            PatternVariantNestedForDestructureObjectAst::SkipNArgs(skip_n_args) => skip_n_args.get_pos(),
            PatternVariantNestedForDestructureObjectAst::SingleIdentifier(single_identifier) => single_identifier.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PatternVariantNestedForDestructureObjectAst::AttrBind(attr_bind) => attr_bind.get_final_pos(),
            PatternVariantNestedForDestructureObjectAst::SkipNArgs(skip_n_args) => skip_n_args.get_final_pos(),
            PatternVariantNestedForDestructureObjectAst::SingleIdentifier(single_identifier) => single_identifier.get_final_pos(),
        }
    }
}

impl Ast for PatternVariantNestedForAttributeBindingAst {
    fn get_pos(&self) -> usize {
        match self {
            PatternVariantNestedForAttributeBindingAst::DestructureArray(destructure_array) => destructure_array.get_pos(),
            PatternVariantNestedForAttributeBindingAst::DestructureTuple(destructure_tuple) => destructure_tuple.get_pos(),
            PatternVariantNestedForAttributeBindingAst::DestructureObject(destructure_object) => destructure_object.get_pos(),
            PatternVariantNestedForAttributeBindingAst::Literal(literal) => literal.get_pos(),
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            PatternVariantNestedForAttributeBindingAst::DestructureArray(destructure_array) => destructure_array.get_final_pos(),
            PatternVariantNestedForAttributeBindingAst::DestructureTuple(destructure_tuple) => destructure_tuple.get_final_pos(),
            PatternVariantNestedForAttributeBindingAst::DestructureObject(destructure_object) => destructure_object.get_final_pos(),
            PatternVariantNestedForAttributeBindingAst::Literal(literal) => literal.get_final_pos(),
        }
    }
}

impl ToBinaryExpression for PatternVariantAst {
    fn to_binary_expression(pos: usize, lhs: ExpressionAst, op: TokenAst, rhs: Self) -> ExpressionAst {
        ExpressionAst::Primary(PrimaryExpressionAst::Is(IsExpressionAst::new(pos, Box::new(lhs), op, Box::new(rhs))))
    }
}
