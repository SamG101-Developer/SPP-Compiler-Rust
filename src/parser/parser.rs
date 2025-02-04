use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::assignment_statement_ast::AssignmentStatementAst;
use crate::asts::ast::ToBinaryExpression;
use crate::asts::case_expression_ast::CaseExpressionAst;
use crate::asts::case_expression_branch_ast::CaseExpressionBranchAst;
use crate::asts::class_attribute_ast::ClassAttributeAst;
use crate::asts::class_implementation_ast::ClassImplementationAst;
use crate::asts::class_member_ast::ClassMemberAst;
use crate::asts::class_prototype_ast::ClassPrototypeAst;
use crate::asts::convention_ast::ConventionAst;
use crate::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::function_call_argument::FunctionCallArgumentAst;
use crate::asts::function_call_argument_group_ast::FunctionCallArgumentGroupAst;
use crate::asts::function_implementation_ast::FunctionImplementationAst;
use crate::asts::function_member_ast::FunctionMemberAst;
use crate::asts::function_parameter_ast::FunctionParameterAst;
use crate::asts::function_parameter_group_ast::FunctionParameterGroupAst;
use crate::asts::function_prototype_ast::{FunctionPrototypeAst, FunctionPrototypeBaseAst};
use crate::asts::gen_expression_ast::GenExpressionAst;
use crate::asts::generic_argument_ast::GenericArgumentAst;
use crate::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::asts::generic_parameter_ast::GenericParameterAst;
use crate::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::asts::global_constant_ast::GlobalConstantAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::let_statement_ast::LetStatementAst;
use crate::asts::literal_ast::LiteralAst;
use crate::asts::local_variable_ast::{
    LocalVariableAst, LocalVariableNestedForAttributeBindingAst,
    LocalVariableNestedForDestructureArrayAst, LocalVariableNestedForDestructureObjectAst,
    LocalVariableNestedForDestructureTupleAst,
};
use crate::asts::local_variable_attribute_binding_ast::LocalVariableAttributeBindingAst;
use crate::asts::local_variable_destructure_array_ast::LocalVariableDestructureArrayAst;
use crate::asts::local_variable_destructure_object_ast::LocalVariableDestructureObjectAst;
use crate::asts::local_variable_destructure_skip_1_argument_ast::LocalVariableDestructureSkip1ArgumentAst;
use crate::asts::local_variable_destructure_skip_n_arguments_ast::LocalVariableDestructureSkipNArgumentsAst;
use crate::asts::local_variable_destructure_tuple_ast::LocalVariableDestructureTupleAst;
use crate::asts::local_variable_single_identifier_alias_ast::LocalVariableSingleIdentifierAliasAst;
use crate::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use crate::asts::loop_condition_ast::LoopConditionAst;
use crate::asts::loop_control_flow_statement_ast::LoopControlFlowStatementAst;
use crate::asts::loop_control_flow_statement_final_part_ast::LoopControlFlowStatementFinalPartAst;
use crate::asts::loop_else_statement_ast::LoopElseStatementAst;
use crate::asts::loop_expression_ast::LoopExpressionAst;
use crate::asts::module_implementation_ast::ModuleImplementationAst;
use crate::asts::module_member_ast::ModuleMemberAst;
use crate::asts::module_prototype_ast::ModulePrototypeAst;
use crate::asts::object_initializer::ObjectInitializerAst;
use crate::asts::object_initializer_argument_ast::ObjectInitializerArgumentAst;
use crate::asts::object_initializer_argument_group_ast::ObjectInitializerArgumentGroupAst;
use crate::asts::parenthesized_expression_ast::ParenthesizedExpressionAst;
use crate::asts::pattern_guard_ast::PatternGuardAst;
use crate::asts::pattern_variant_ast::{
    PatternVariantAst, PatternVariantNestedForAttributeBindingAst,
    PatternVariantNestedForDestructureArrayAst, PatternVariantNestedForDestructureObjectAst,
    PatternVariantNestedForDestructureTupleAst,
};
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
use crate::asts::pin_statement_ast::PinStatementAst;
use crate::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;
use crate::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;
use crate::asts::postfix_expression_operator_step_keyword_ast::PostfixExpressionOperatorStepKeywordAst;
use crate::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::asts::rel_statement_ast::RelStatementAst;
use crate::asts::ret_statement_ast::RetStatementAst;
use crate::asts::statement_ast::StatementAst;
use crate::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::asts::sup_implementation_ast::SupImplementationAst;
use crate::asts::sup_member_ast::SupMemberAst;
use crate::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::asts::sup_use_statement_ast::SupUseStatementAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;
use crate::asts::type_optional_ast::TypeOptionalAst;
use crate::asts::type_tuple_ast::TypeTupleAst;
use crate::asts::type_variant_ast::TypeVariantAst;
use crate::asts::unary_expression_ast::UnaryExpressionAst;
use crate::asts::unary_expression_operator_ast::UnaryExpressionOperatorAst;
use crate::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;
use crate::asts::use_statement_ast::UseStatementAst;
use crate::asts::where_block_ast::WhereBlockAst;
use crate::asts::where_constraints_ast::WhereConstraintsAst;
use crate::asts::where_constraints_group_ast::WhereConstraintsGroupAst;
use crate::asts::with_expression_alias_ast::WithExpressionAliasAst;
use crate::asts::with_expression_ast::WithExpressionAst;
use crate::lexer::token::{Keywords, TokenStream, TokenType};
use crate::parser::parser_error::SyntaxError;
use std::cell::RefCell;
use std::rc::Rc;

type ParserResult<T> = Result<T, SyntaxError>;

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: TokenStream,
    pub index: usize,
    token_len: usize,
    error: SyntaxError,
}

impl Parser {
    fn current_pos(&self) -> usize {
        self.index
    }

    fn store_error(&mut self, pos: usize, message: String) -> bool {
        if pos > self.error.pos {
            self.error.expected_tokens.clear();
            self.error.pos = pos;
            self.error.message = message;
            true
        } else {
            false
        }
    }
}

macro_rules! parse_once {
    ($self:ident, $method:expr) => {
        $method()?
    };
}

macro_rules! parse_optional {
    ($self:ident, $method:expr) => {{
        let index = $self.index;
        let result = $method();

        match result {
            Ok(ast) => Some(ast),
            Err(_) => {
                $self.index = index;
                None
            }
        }
    }};
}

macro_rules! parse_zero_or_more {
    ($self:ident, $method:expr, $sep:expr) => {{
        let mut result = vec![];
        loop {
            let index = $self.index;
            let one_result = $method();
            match one_result {
                Ok(ast) => {
                    result.push(ast);
                    let ast = parse_optional!($self, $sep);
                    if ast.is_some() {
                        break result;
                    }
                }
                Err(_) => {
                    $self.index = index;
                    break result;
                }
            }
        }
    }};
}

// use parse_zero_or_more then check if the result is empty
macro_rules! parse_one_or_more {
    ($self:ident, $method:expr, $sep:expr) => {{
        let result = parse_zero_or_more!($self, $method, $sep);
        if result.is_empty() {
            Err(SyntaxError::new(
                $self.current_pos(),
                "Expected at least one".to_string(),
            ))
        } else {
            Ok(result)
        }
    }?};
}

macro_rules! parse_alternate {
    ($self:ident, $method:expr) => {
        $method()?
    };
    ($self:ident, $method:expr, $($rest:expr),+) => {
        match parse_optional!($self, $method) {
            Some(p) => p,
            None => parse_alternate!($self, $($rest),+),
        }
    };
}

impl Parser {
    fn parse_root(&mut self) -> ParserResult<ModulePrototypeAst> {
        let p1 = parse_once!(self, || self.parse_module_prototype());
        let p2 = parse_once!(self, || self.parse_eof());
        Ok(p1)
    }

    fn parse_module_prototype(&mut self) -> ParserResult<ModulePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_module_implementation());
        Ok(ModulePrototypeAst::new(c1, p1))
    }

    fn parse_module_implementation(&mut self) -> ParserResult<ModuleImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_module_member(), || self
            .parse_token_no_token());
        Ok(ModuleImplementationAst::new(c1, p1))
    }

    fn parse_module_member(&mut self) -> ParserResult<ModuleMemberAst> {
        let p1 = parse_alternate!(
            self,
            || ModuleMemberAst::Function(parse_once!(self, || self.parse_function_prototype())),
            || ModuleMemberAst::GlobalConst(parse_once!(self, || self.parse_global_constant())),
            || ModuleMemberAst::UseStatement(parse_once!(self, || self.parse_global_use_statement())),
            || ModuleMemberAst::Class(parse_once!(self, || self.parse_class_prototype())),
            || ModuleMemberAst::SupExtension(parse_once!(self, || self.parse_sup_prototype_extension())),
            || ModuleMemberAst::SupFunctions(parse_once!(self, || self.parse_sup_prototype_functions()))
        );
        Ok(p1)
    }

    fn parse_class_prototype(&mut self) -> ParserResult<ClassPrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::Cls));
        let p3 = parse_once!(self, || self.parse_upper_identifier());
        let p4 = parse_optional!(self, || self.parse_generic_parameters());
        let p5 = parse_optional!(self, || self.parse_where_block());
        let p6 = parse_once!(self, || self.parse_class_implementation());
        Ok(ClassPrototypeAst::new(
            c1,
            p1,
            p2,
            TypeAst::from(p3),
            p4,
            p5,
            p6,
        ))
    }

    fn parse_class_implementation(&mut self) -> ParserResult<ClassImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_curly_brace());
        let p2 = parse_zero_or_more!(self, || self.parse_class_member(), || self.parse_token_no_token());
        let p3 = parse_once!(self, || self.parse_token_right_curly_brace());
        Ok(ClassImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_class_member(&mut self) -> ParserResult<ClassMemberAst> {
        let p1 = parse_once!(self, || self.parse_class_attribute());
        Ok(ClassMemberAst::Attr(p1))
    }

    fn parse_class_attribute(&mut self) -> ParserResult<ClassAttributeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let p2 = parse_once!(self, || self.parse_identifier());
        let p3 = parse_once!(self, || self.parse_token_colon());
        let p4 = parse_once!(self, || self.parse_type());
        Ok(ClassAttributeAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_sup_prototype_functions(&mut self) -> ParserResult<SupPrototypeFunctionsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Sup));
        let p2 = parse_optional!(self, || self.parse_generic_parameters());
        let p3 = parse_once!(self, || self.parse_type());
        let p4 = parse_optional!(self, || self.parse_where_block());
        let p5 = parse_once!(self, || self.parse_sup_implementation());
        Ok(SupPrototypeFunctionsAst::new(c1, p1, p2, p3, p4, p5))
    }

    fn parse_sup_prototype_extension(&mut self) -> ParserResult<SupPrototypeExtensionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Sup));
        let p2 = parse_optional!(self, || self.parse_generic_parameters());
        let p3 = parse_once!(self, || self.parse_type());
        let p4 = parse_once!(self, || self.parse_keyword(Keywords::Ext));
        let p5 = parse_once!(self, || self.parse_type());
        let p6 = parse_optional!(self, || self.parse_where_block());
        let p7 = parse_once!(self, || self.parse_sup_implementation());
        Ok(SupPrototypeExtensionAst::new(
            c1, p1, p2, p3, p4, p5, p6, p7,
        ))
    }

    fn parse_sup_implementation(&mut self) -> ParserResult<SupImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_curly_brace());
        let p2 = parse_zero_or_more!(self, || self.parse_sup_member(), || self
            .parse_token_no_token());
        let p3 = parse_once!(self, || self.parse_token_right_curly_brace());
        Ok(SupImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_sup_member(&mut self) -> ParserResult<SupMemberAst> {
        let p1 = parse_alternate!(
            self,
            || SupMemberAst::Method(parse_once!(self, || self.parse_sup_method_prototype())),
            || SupMemberAst::Typedef(parse_once!(self, || self.parse_sup_use_statement()))
        );
        Ok(p1)
    }

    fn parse_sup_method_prototype(&mut self) -> ParserResult<FunctionPrototypeAst> {
        let p1 = parse_once!(self, || self.parse_function_prototype());
        Ok(p1)
    }

    fn parse_sup_use_statement(&mut self) -> ParserResult<SupUseStatementAst> {
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self
            .parse_token_newline());
        let mut p2 = parse_once!(self, || self.parse_use_statement());
        p2.annotations = p1;
        Ok(p2)
    }

    fn parse_function_prototype(&mut self) -> ParserResult<FunctionPrototypeAst> {
        let p1 = parse_alternate!(
            self,
            || FunctionPrototypeAst::Subroutine(parse_once!(self, || self.parse_subroutine_prototype())),
            || FunctionPrototypeAst::Coroutine(parse_once!(self, || self.parse_coroutine_prototype()))
        );
        Ok(p1)
    }

    fn parse_subroutine_prototype(&mut self) -> ParserResult<SubroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::Fun));
        let p3 = parse_once!(self, || self.parse_identifier());
        let p4 = parse_optional!(self, || self.parse_generic_parameters());
        let p5 = parse_once!(self, || self.parse_function_parameters());
        let p6 = parse_once!(self, || self.parse_token_rightward_arrow());
        let p7 = parse_once!(self, || self.parse_type());
        let p8 = parse_optional!(self, || self.parse_where_block());
        let p9 = parse_once!(self, || self.parse_function_implementation());
        Ok(SubroutinePrototypeAst(FunctionPrototypeBaseAst::new(
            c1, p1, p2, p3, p4, p5, p6, p7, p8, p9,
        )))
    }

    fn parse_coroutine_prototype(&mut self) -> ParserResult<CoroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::Cor));
        let p3 = parse_once!(self, || self.parse_identifier());
        let p4 = parse_optional!(self, || self.parse_generic_parameters());
        let p5 = parse_once!(self, || self.parse_function_parameters());
        let p6 = parse_once!(self, || self.parse_token_rightward_arrow());
        let p7 = parse_once!(self, || self.parse_type());
        let p8 = parse_optional!(self, || self.parse_where_block());
        let p9 = parse_once!(self, || self.parse_function_implementation());
        Ok(CoroutinePrototypeAst(FunctionPrototypeBaseAst::new(
            c1, p1, p2, p3, p4, p5, p6, p7, p8, p9,
        )))
    }

    fn parse_function_implementation(&mut self) -> ParserResult<FunctionImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_curly_brace());
        let p2 = parse_zero_or_more!(self, || self.parse_function_member(), || self.parse_token_no_token());
        let p3 = parse_once!(self, || self.parse_token_right_curly_brace());
        Ok(FunctionImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_function_member(&mut self) -> ParserResult<FunctionMemberAst> {
        let p1 = parse_once!(self, || self.parse_statement());
        Ok(p1)
    }

    fn parse_function_call_arguments(&mut self) -> ParserResult<FunctionCallArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_parenthesis());
        let p2 = parse_zero_or_more!(self, || self.parse_function_call_argument(), || self.parse_token_comma());
        let p3 = parse_once!(self, || self.parse_token_right_parenthesis());
        Ok(FunctionCallArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_function_call_argument(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let p1 = parse_alternate!(
            self,
            || parse_once!(self, self.parse_function_call_argument_named()),
            || parse_once!(self, self.parse_function_call_argument_unnamed())
        );
        Ok(p1)
    }

    fn parse_function_call_argument_unnamed(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_convention());
        let p2 = parse_optional!(self, || self.parse_token_double_dot());
        let p3 = parse_once!(self, || self.parse_expression());
        Ok(FunctionCallArgumentAst::new_unnamed_argument(
            c1, p1, p2, p3,
        ))
    }

    fn parse_function_call_argument_named(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_identifier());
        let p2 = parse_once!(self, || self.parse_token_assign());
        let p3 = parse_once!(self, || self.parse_convention());
        let p4 = parse_once!(self, || self.parse_expression());
        Ok(FunctionCallArgumentAst::new_named_argument(
            c1, p1, p2, p3, p4,
        ))
    }

    fn parse_function_parameters(&mut self) -> ParserResult<FunctionParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_parenthesis());
        let p2 = parse_zero_or_more!(self, || self.parse_function_parameter(), || self
            .parse_token_comma());
        let p3 = parse_once!(self, || self.parse_token_right_parenthesis());
        Ok(FunctionParameterGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_function_parameter(&mut self) -> ParserResult<FunctionParameterAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_function_parameter_variadic(),
            || self.parse_function_parameter_optional(),
            || self.parse_function_parameter_required(),
            || self.parse_function_parameter_self()
        );
        Ok(p1)
    }

    fn parse_function_parameter_self(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, || self.parse_keyword(Keywords::Mut));
        let p2 = parse_once!(self, || self.parse_convention());
        let p3 = parse_once!(self, || self.parse_self_keyword());
        Ok(FunctionParameterAst::new_self(c1, p1, p2, p3))
    }

    fn parse_function_parameter_required(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_local_variable());
        let p2 = parse_once!(self, || self.parse_token_primitive(TokenType::TkColon));
        let p3 = parse_once!(self, || self.parse_convention());
        let p4 = parse_once!(self, || self.parse_type());
        Ok(FunctionParameterAst::new_required(c1, p1, p2, p3, p4))
    }

    fn parse_function_parameter_optional(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_local_variable());
        let p2 = parse_once!(self, || self.parse_token_primitive(TokenType::TkColon));
        let p3 = parse_once!(self, || self.parse_convention());
        let p4 = parse_once!(self, || self.parse_type());
        let p5 = parse_once!(self, || self.parse_token_assign());
        let p6 = parse_once!(self, || self.parse_expression());
        Ok(FunctionParameterAst::new_optional(
            c1, p1, p2, p3, p4, p5, p6,
        ))
    }

    fn parse_function_parameter_variadic(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_double_dot());
        let p2 = parse_once!(self, || self.parse_local_variable());
        let p3 = parse_once!(self, || self.parse_token_primitive(TokenType::TkColon));
        let p4 = parse_once!(self, || self.parse_convention());
        let p5 = parse_once!(self, || self.parse_type());
        Ok(FunctionParameterAst::new_variadic(c1, p1, p2, p3, p4, p5))
    }

    fn parse_generic_arguments(&mut self) -> ParserResult<GenericArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_square_bracket());
        let p2 = parse_zero_or_more!(self, || self.parse_generic_argument(), || self
            .parse_token_comma());
        let p3 = parse_once!(self, || self.parse_token_right_square_bracket());
        Ok(GenericArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_generic_argument(&mut self) -> ParserResult<GenericArgumentAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_generic_type_argument_named(),
            || self.parse_generic_type_argument_unnamed(),
            || self.parse_generic_comp_argument_named(),
            || self.parse_generic_comp_argument_unnamed()
        );
        Ok(p1)
    }

    fn parse_generic_type_argument_named(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_upper_identifier());
        let p2 = parse_once!(self, || self.parse_token_assign());
        let p3 = parse_once!(self, || self.parse_type());
        Ok(GenericArgumentAst::new_type_named(
            c1,
            TypeAst::from(p1),
            p2,
            p3,
        ))
    }

    fn parse_generic_type_argument_unnamed(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_type());
        Ok(GenericArgumentAst::new_type_unnamed(c1, p1))
    }

    fn parse_generic_comp_argument_named(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_identifier());
        let p2 = parse_once!(self, || self.parse_token_assign());
        let p3 = parse_once!(self, || self.parse_global_constant_value());
        Ok(GenericArgumentAst::new_comp_named(
            c1,
            TypeAst::from(p1),
            p2,
            p3,
        ))
    }

    fn parse_generic_comp_argument_unnamed(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_global_constant_value());
        Ok(GenericArgumentAst::new_comp_unnamed(c1, p1))
    }

    fn parse_generic_parameters(&mut self) -> ParserResult<GenericParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_square_bracket());
        let p2 = parse_zero_or_more!(self, || self.parse_generic_parameter(), || self
            .parse_token_comma());
        let p3 = parse_once!(self, || self.parse_token_right_square_bracket());
        Ok(GenericParameterGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_generic_parameter(&mut self) -> ParserResult<GenericParameterAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_generic_comp_parameter_variadic(),
            || self.parse_generic_comp_parameter_optional(),
            || self.parse_generic_comp_parameter_required(),
            || self.parse_generic_type_parameter_variadic(),
            || self.parse_generic_type_parameter_optional(),
            || self.parse_generic_type_parameter_required()
        );
        Ok(p1)
    }

    fn parse_generic_type_parameter_required(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_upper_identifier());
        let p2 = parse_optional!(self, || self.parse_generic_inline_constraints());
        Ok(GenericParameterAst::new_type_required(
            c1,
            TypeAst::from(p1),
            p2,
        ))
    }

    fn parse_generic_type_parameter_optional(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_upper_identifier());
        let p2 = parse_optional!(self, || self.parse_generic_inline_constraints());
        let p3 = parse_once!(self, || self.parse_token_assign());
        let p4 = parse_once!(self, || self.parse_type());
        Ok(GenericParameterAst::new_type_optional(
            c1,
            TypeAst::from(p1),
            p2,
            p3,
            p4,
        ))
    }

    fn parse_generic_type_parameter_variadic(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_double_dot());
        let p2 = parse_once!(self, || self.parse_upper_identifier());
        let p3 = parse_optional!(self, || self.parse_generic_inline_constraints());
        Ok(GenericParameterAst::new_type_variadic(
            c1,
            p1,
            TypeAst::from(p2),
            p3,
        ))
    }

    fn parse_generic_comp_parameter_required(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Cmp));
        let p2 = parse_once!(self, || self.parse_identifier());
        let p3 = parse_once!(self, || self.parse_token_colon());
        let p4 = parse_once!(self, || self.parse_type());
        Ok(GenericParameterAst::new_comp_required(
            c1,
            p1,
            TypeAst::from(p2),
            p3,
            p4,
        ))
    }

    fn parse_generic_comp_parameter_optional(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Cmp));
        let p2 = parse_once!(self, || self.parse_identifier());
        let p3 = parse_once!(self, || self.parse_token_colon());
        let p4 = parse_once!(self, || self.parse_type());
        let p5 = parse_once!(self, || self.parse_token_assign());
        let p6 = parse_once!(self, || self.parse_global_constant_value());
        Ok(GenericParameterAst::new_comp_optional(
            c1,
            p1,
            TypeAst::from(p2),
            p3,
            p4,
            p5,
            p6,
        ))
    }

    fn parse_generic_comp_parameter_variadic(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Cmp));
        let p2 = parse_once!(self, || self.parse_token_double_dot());
        let p3 = parse_once!(self, || self.parse_identifier());
        let p4 = parse_once!(self, || self.parse_token_colon());
        let p5 = parse_once!(self, || self.parse_type());
        Ok(GenericParameterAst::new_comp_variadic(
            c1,
            p1,
            p2,
            TypeAst::from(p3),
            p4,
            p5,
        ))
    }

    fn parse_generic_inline_constraints(&mut self) -> ParserResult<GenericParameterConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_colon());
        let p2 = parse_one_or_more!(self, || self.parse_type(), || self.parse_token_comma());
        Ok(GenericParameterConstraintsAst::new(c1, p1, p2))
    }

    fn parse_where_block(&mut self) -> ParserResult<WhereBlockAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Where));
        let p2 = parse_once!(self, || self.parse_where_block_constraints_group());
        Ok(WhereBlockAst::new(c1, p1, p2))
    }

    fn parse_where_block_constraints_group(&mut self) -> ParserResult<WhereConstraintsGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_square_bracket());
        let p2 = parse_one_or_more!(self, || self.parse_where_block_constraints(), || self
            .parse_token_comma());
        let p3 = parse_once!(self, || self.parse_token_right_square_bracket());
        Ok(WhereConstraintsGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_where_block_constraints(&mut self) -> ParserResult<WhereConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, || self.parse_type(), || self.parse_token_comma());
        let p2 = parse_once!(self, || self.parse_token_colon());
        let p3 = parse_once!(self, || self.parse_type());
        Ok(WhereConstraintsAst::new(c1, p1, p2, p3))
    }

    fn parse_annotation(&mut self) -> ParserResult<AnnotationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_primitive(TokenType::TkAt));
        let p2 = parse_once!(self, || self.parse_identifier());
        Ok(AnnotationAst::new(c1, p1, p2))
    }

    fn parse_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_once!(self, || self.parse_binary_expression_precedence_level_1());
        Ok(p1)
    }

    fn parse_binary_expression_precedence_level_n_rhs<T: ToBinaryExpression + 'static>(&mut self, op: fn(&mut Parser) -> ParserResult<TokenAst>, rhs: fn(&mut Parser) -> ParserResult<ExpressionAst>,
    ) -> ParserResult<(TokenAst, T)> {
        let p1 = parse_once!(self, || op(self));
        let p2 = parse_once!(self, || rhs(self));
        Ok((p1, p2))
    }

    fn parse_binary_expression_precedence_level_n<T: ToBinaryExpression + 'static>(&mut self, lhs: fn(&mut Parser) -> ParserResult<ExpressionAst>, op: fn(&mut Parser) -> ParserResult<TokenAst>, rhs: fn(&mut Parser) -> ParserResult<ExpressionAst>) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || lhs(self));
        let p2 = parse_optional!(self, || self
            .parse_binary_expression_precedence_level_n_rhs(op, rhs));

        Ok(if let Some(p2) = p2 {
            T::to_binary_expression(c1, p1, p2.0, p2.1)
        } else {
            p1
        })
    }

    fn parse_binary_expression_precedence_level_1(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_2,
            Parser::parse_binary_op_precedence_level_1,
            Parser::parse_binary_expression_precedence_level_1)
    }

    fn parse_binary_expression_precedence_level_2(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_3,
            Parser::parse_binary_op_precedence_level_2,
            Parser::parse_binary_expression_precedence_level_2)
    }

    fn parse_binary_expression_precedence_level_3(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_4,
            Parser::parse_binary_op_precedence_level_3,
            Parser::parse_pattern_group_destructure)
    }

    fn parse_binary_expression_precedence_level_4(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_5,
            Parser::parse_binary_op_precedence_level_4,
            Parser::parse_binary_expression_precedence_level_4)
    }

    fn parse_binary_expression_precedence_level_5(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_6,
            Parser::parse_binary_op_precedence_level_5,
            Parser::parse_binary_expression_precedence_level_5)
    }

    fn parse_binary_expression_precedence_level_6(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_unary_expression,
            Parser::parse_binary_op_precedence_level_6,
            Parser::parse_binary_expression_precedence_level_6)
    }

    fn parse_unary_expression(&mut self) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_unary_op(), || self.parse_token_no_token());
        let p2 = parse_once!(self, || self.parse_postfix_expression());
        p1.into_iter().rev().fold(Ok(p2), |acc, x| { Ok(ExpressionAst::Unary(UnaryExpressionAst::new(c1, x, Box::from(acc?), ))) })
    }

    fn parse_postfix_expression(&mut self) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_primary_expression());
        let p2 = parse_zero_or_more!(self, || self.parse_postfix_op(), || self.parse_token_no_token());
        p2.into_iter().fold(Ok(p1), |acc, x| { Ok(ExpressionAst::Postfix(PostfixExpressionAst::new(c1, Box::from(acc?), x, ))) })
    }

    fn parse_primary_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_alternate!(
            self,
            || ExpressionAst::Primary(PrimaryExpressionAst::Literal(self.parse_literal())),
            || ExpressionAst::Primary(PrimaryExpressionAst::ObjectInitializer(self.parse_object_initializer())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Parenthesized(self.parse_parenthesized_expression())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Type(self.parse_type())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Identifier(self.parse_identifier())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Case(self.parse_case_expression())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Loop(self.parse_loop_expression())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Gen(self.parse_gen_expression())),
            || ExpressionAst::Primary(PrimaryExpressionAst::With(self.parse_with_expression())),
            || ExpressionAst::Primary(PrimaryExpressionAst::InnerScope(self.parse_inner_scope())),
            || ExpressionAst::Primary(PrimaryExpressionAst::SelfIdentifier(self.parse_self_keyword())),
            || ExpressionAst::Primary(PrimaryExpressionAst::Fold(self.parse_token_double_dot())));
        Ok(p1)
    }

    fn parse_parenthesized_expression(&mut self) -> ParserResult<ParenthesizedExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_left_parenthesis());
        let p2 = parse_once!(self, || self.parse_expression());
        let p3 = parse_once!(self, || self.parse_token_right_parenthesis());
        Ok(ParenthesizedExpressionAst::new(c1, p1, Box::from(p2), p3))
    }

    fn parse_self_keyword(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::SelfVal));
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    fn parse_case_expression(&mut self) -> ParserResult<CaseExpressionAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_case_expression_patterns(),
            || self.parse_case_expression_simple()
        );
        Ok(p1)
    }

    fn parse_case_expression_patterns(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Case));
        let p2 = parse_once!(self, || self.parse_expression());
        let p3 = parse_once!(self, || self.parse_keyword(Keywords::Of));
        let p4 = parse_one_or_more!(self, || self.parse_case_expression_branch(), || self.parse_token_no_token());
        Ok(CaseExpressionAst::new(c1, p1, Box::from(p2), p3, p4))
    }

    fn parse_case_expression_simple(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Case));
        let p2 = parse_once!(self, || self.parse_expression());
        let p3 = parse_once!(self, || self.parse_inner_scope());
        let p4 = parse_zero_or_more!(self, || self.parse_case_expression_branch_simple(), || self.parse_token_no_token());
        Ok(CaseExpressionAst::new_from_simple(c1, p1, Box::from(p2), p3, p4))
    }

    fn parse_loop_expression(&mut self) -> ParserResult<LoopExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Loop));
        let p2 = parse_once!(self, || self.parse_loop_expression_condition());
        let p3 = parse_once!(self, || self.parse_inner_scope());
        let p4 = parse_optional!(self, || self.parse_loop_else_statement());
        Ok(LoopExpressionAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_loop_expression_condition(&mut self) -> ParserResult<LoopConditionAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_loop_expression_condition_iterable(),
            || self.parse_loop_expression_condition_boolean()
        );
        Ok(p1)
    }

    fn parse_loop_expression_condition_boolean(&mut self) -> ParserResult<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_expression());
        Ok(LoopConditionAst::new_boolean(c1, Box::new(p1)))
    }

    fn parse_loop_expression_condition_iterable(&mut self) -> ParserResult<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_local_variable());
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::In));
        let p3 = parse_once!(self, || self.parse_expression());
        Ok(LoopConditionAst::new_iterable(c1, p1, p2, Box::new(p3)))
    }

    fn parse_loop_else_statement(&mut self) -> ParserResult<LoopElseStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Else));
        let p2 = parse_once!(self, || self.parse_inner_scope());
        Ok(LoopElseStatementAst::new(c1, p1, p2))
    }

    fn parse_gen_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_gen_expression_unroll(),
            || self.parse_gen_expression_normal()
        );
        Ok(p1)
    }

    fn parse_gen_expression_normal(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_gen_expression_normal_with_expression(),
            || self.parse_gen_expression_normal_no_expression()
        );
        Ok(p1)
    }

    fn parse_gen_expression_normal_no_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Gen));
        Ok(GenExpressionAst::new(c1, p1, None, ConventionAst::Mov { pos: c1 }, None, ))
    }

    fn parse_gen_expression_normal_with_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Gen));
        let p2 = parse_once!(self, || self.parse_convention());
        let p3 = parse_once!(self, || self.parse_expression());
        Ok(GenExpressionAst::new(c1, p1, None, p2, Some(Box::new(p3))))
    }

    fn parse_gen_expression_unroll(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Gen));
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::With));
        let p3 = parse_once!(self, || self.parse_expression());
        Ok(GenExpressionAst::new(c1, p1, Some(p2), ConventionAst::Mov { pos: c1 }, Some(Box::new(p3))))

    }

    fn parse_with_expression(&mut self) -> ParserResult<WithExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::With));
        let p2 = parse_optional!(self, || self.parse_with_expression_lhs_alias());
        let p3 = parse_once!(self, || self.parse_expression());
        let p4 = parse_once!(self, || self.parse_inner_scope());
        Ok(WithExpressionAst::new(c1, p1, p2, Box::new(p3), p4))
    }

    fn parse_with_expression_lhs_alias(&mut self) -> ParserResult<WithExpressionAliasAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_local_variable());
        let p2 = parse_once!(self, || self.parse_token_assign());
        Ok(WithExpressionAliasAst::new(c1, p1, p2))
    }

    fn parse_ret_statement(&mut self) -> ParserResult<RetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Ret));
        let p2 = parse_optional!(self, || self.parse_expression());
        Ok(RetStatementAst::new(c1, p1, p2))
    }

    fn parse_exit_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, || self.parse_keyword(Keywords::Exit), || self.parse_token_no_token());
        let p2 = parse_optional!(self, || self.parse_exit_statement_final_action());
        return Ok(LoopControlFlowStatementAst::new(c1, p1, p2));
    }

    fn parse_exit_statement_final_action(&mut self, ) -> ParserResult<LoopControlFlowStatementFinalPartAst> {
        let p1 = parse_alternate!(
            self,
            || LoopControlFlowStatementFinalPartAst::Skip(self.parse_keyword(Keywords::Skip)),
            || LoopControlFlowStatementFinalPartAst::Expression(self.parse_expression()));
        Ok(p1)
    }

    fn parse_skip_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, || self.parse_keyword(Keywords::Skip), || self.parse_token_no_token());
        Ok(LoopControlFlowStatementAst::new(c1, p1, None))
    }

    fn parse_pin_statement(&mut self) -> ParserResult<PinStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Pin));
        let p2 = parse_one_or_more!(self, || self.parse_expression(), || self.parse_token_comma());
        Ok(PinStatementAst::new(c1, p1, p2))
    }

    fn parse_rel_statement(&mut self) -> ParserResult<RelStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Rel));
        let p2 = parse_one_or_more!(self, || self.parse_expression(), || self.parse_token_comma());
        Ok(RelStatementAst::new(c1, p1, p2))
    }

    fn parse_inner_scope(&mut self) -> ParserResult<InnerScopeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_primitive(TokenType::TkLeftCurlyBrace));
        let p2 = parse_zero_or_more!(self, || self.parse_statement(), || self.parse_token_no_token());
        let p3 = parse_once!(self, || self.parse_token_primitive(TokenType::TkRightCurlyBrace));
        Ok(InnerScopeAst::new(c1, p1, p2, p3))
    }

    fn parse_statement(&mut self) -> ParserResult<StatementAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_use_statement().enum_wrapper(Box::new(StatementAst::Use)),
            || self.parse_let_statement().enum_wrapper(Box::new(StatementAst::Let)),
            || self.parse_ret_statement().enum_wrapper(Box::new(StatementAst::Ret)),
            || self.parse_exit_statement().enum_wrapper(Box::new(StatementAst::LoopControlFlow)),
            || self.parse_skip_statement().enum_wrapper(Box::new(StatementAst::LoopControlFlow)),
            || self.parse_pin_statement().enum_wrapper(Box::new(StatementAst::Pin)),
            || self.parse_rel_statement().enum_wrapper(Box::new(StatementAst::Rel)),
            || self.parse_assignment_statement().enum_wrapper(Box::new(StatementAst::Assignment)),
            || self.parse_expression().enum_wrapper(Box::new(StatementAst::Expression)));
        Ok(p1)
    }

    fn parse_global_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let mut p2 = parse_once!(self, || self.parse_use_statement());
        p2.annotations = p1;
        Ok(p2)
    }

    fn parse_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Use));
        let p2 = parse_once!(self, || self.parse_upper_identifier());
        let p3 = parse_optional!(self, || self.parse_generic_parameters());
        let p4 = parse_once!(self, || self.parse_token_assign());
        let p5 = parse_once!(self, || self.parse_type());
        Ok(UseStatementAst::new(c1, vec![], p1, TypeAst::from(p2), p3, p4, p5))
    }

    fn parse_global_constant(&mut self) -> ParserResult<GlobalConstantAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, || self.parse_annotation(), || self.parse_token_newline());
        let p2 = parse_once!(self, || self.parse_keyword(Keywords::Cmp));
        let p3 = parse_once!(self, || self.parse_identifier());
        let p4 = parse_once!(self, || self.parse_token_primitive(TokenType::TkColon));
        let p5 = parse_once!(self, || self.parse_type());
        let p6 = parse_once!(self, || self.parse_token_assign());
        let p7 = parse_once!(self, || self.parse_global_constant_value());
        Ok(GlobalConstantAst::new(c1, p1, p2, p3, p4, p5, p6, p7))
    }

    fn parse_let_statement(&mut self) -> ParserResult<LetStatementAst> {
        let p1 = parse_alternate!(
            self,
            || self.parse_let_statement_initialized(),
            || self.parse_let_statement_uninitialized());
        Ok(p1)
    }

    fn parse_let_statement_initialized(&mut self) -> ParserResult<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Let));
        let p2 = parse_once!(self, || self.parse_local_variable());
        let p3 = parse_once!(self, || self.parse_token_assign());
        let p4 = parse_once!(self, || self.parse_expression());
        Ok(LetStatementAst::new_initialized(c1, p1, p2, p3, p4))
    }

    fn parse_let_statement_uninitialized(&mut self) -> ParserResult<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_keyword(Keywords::Let));
        let p2 = parse_once!(self, || self.parse_local_variable());
        let p3 = parse_once!(self, || self.parse_token_primitive(TokenType::TkColon));
        let p4 = parse_once!(self, || self.parse_type());
        Ok(LetStatementAst::new_uninitialized(c1, p1, p2, p3, p4))
    }

    fn parse_local_variable(&mut self) -> ParserResult<LocalVariableAst> {
        let p1 = parse_alternate!(
            self,
            || LocalVariableAst::DestructureArray(self.parse_local_variable_destructure_array()),
            || LocalVariableAst::DestructureTuple(self.parse_local_variable_destructure_tuple()),
            || LocalVariableAst::DestructureObject(self.parse_local_variable_destructure_object()),
            || LocalVariableAst::SingleIdentifier(self.parse_local_variable_single_identifier()));
        Ok(p1)
    }

    fn parse_local_variable_destructure_skip_argument(&mut self) -> ParserResult<LocalVariableDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_underscore());
        Ok(LocalVariableDestructureSkip1ArgumentAst::new(c1, p1))
    }

    fn parse_local_variable_destructure_skip_arguments(&mut self) -> ParserResult<LocalVariableDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, || self.parse_token_double_dot());
        let p2 = parse_optional!(self, || self.parse_local_variable_single_identifier());
        Ok(LocalVariableDestructureSkipNArgumentsAst::new(c1, p1, p2))
    }

    fn parse_local_variable_single_identifier(&mut self) -> ParserResult<LocalVariableSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, || self.parse_keyword(Keywords::Mut));


        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self
            .parse_local_variable_single_identifier_alias()
            .parse_optional();
        return Ok(LocalVariableSingleIdentifierAst::new(c1, p1, p2, p3));
    }

    fn parse_local_variable_single_identifier_alias(
        &mut self,
    ) -> ParserResult<LocalVariableSingleIdentifierAliasAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::As).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(LocalVariableSingleIdentifierAliasAst::new(c1, p1, p2));
    }

    fn parse_local_variable_destructure_array(
        &mut self,
    ) -> ParserResult<LocalVariableDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self
            .parse_local_variable_nested_for_destructure_array()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return Ok(LocalVariableDestructureArrayAst::new(c1, p1, p2, p3));
    }

    fn parse_local_variable_destructure_tuple(
        &mut self,
    ) -> ParserResult<LocalVariableDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self
            .parse_local_variable_nested_for_destructure_tuple()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LocalVariableDestructureTupleAst::new(c1, p1, p2, p3));
    }

    fn parse_local_variable_destructure_object(
        &mut self,
    ) -> ParserResult<LocalVariableDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token_left_parenthesis().parse_once()?;
        let p3 = self
            .parse_local_variable_nested_for_destructure_object()
            .parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LocalVariableDestructureObjectAst::new(c1, p1, p2, p3, p4));
    }

    fn parse_local_variable_attribute_binding(
        &mut self,
    ) -> ParserResult<LocalVariableAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self
            .parse_local_variable_nested_for_attribute_binding()
            .parse_once()?;
        return Ok(LocalVariableAttributeBindingAst::new(c1, p1, p2, p3));
    }

    fn parse_local_variable_nested_for_destructure_array(
        &mut self,
    ) -> ParserResult<LocalVariableNestedForDestructureArrayAst> {
        let p1 = self
            .parse_local_variable_destructure_array()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::DestructureArray,
            ));
        let p2 = self
            .parse_local_variable_destructure_tuple()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::DestructureTuple,
            ));
        let p3 = self
            .parse_local_variable_destructure_object()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::DestructureObject,
            ));
        let p4 = self
            .parse_local_variable_single_identifier()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::SingleIdentifier,
            ));
        let p5 = self
            .parse_local_variable_destructure_skip_arguments()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::SkipNArgs,
            ));
        let p6 = self
            .parse_local_variable_destructure_skip_argument()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureArrayAst::Skip1Args,
            ));
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    fn parse_local_variable_nested_for_destructure_tuple(
        &mut self,
    ) -> ParserResult<LocalVariableNestedForDestructureTupleAst> {
        let p1 = self
            .parse_local_variable_destructure_array()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::DestructureArray,
            ));
        let p2 = self
            .parse_local_variable_destructure_tuple()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::DestructureTuple,
            ));
        let p3 = self
            .parse_local_variable_destructure_object()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::DestructureObject,
            ));
        let p4 = self
            .parse_local_variable_single_identifier()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::SingleIdentifier,
            ));
        let p5 = self
            .parse_local_variable_destructure_skip_arguments()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::SkipNArgs,
            ));
        let p6 = self
            .parse_local_variable_destructure_skip_argument()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureTupleAst::Skip1Args,
            ));
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    fn parse_local_variable_nested_for_destructure_object(
        &mut self,
    ) -> ParserResult<LocalVariableNestedForDestructureObjectAst> {
        let p1 = self
            .parse_local_variable_attribute_binding()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureObjectAst::AttrBind,
            ));
        let p2 = self
            .parse_local_variable_single_identifier()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureObjectAst::SingleIdentifier,
            ));
        let p3 = self
            .parse_local_variable_destructure_skip_arguments()
            .enum_wrapper(Box::new(
                LocalVariableNestedForDestructureObjectAst::SkipNArgs,
            ));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_local_variable_nested_for_attribute_binding(
        &mut self,
    ) -> ParserResult<LocalVariableNestedForAttributeBindingAst> {
        let p1 = self
            .parse_local_variable_destructure_array()
            .enum_wrapper(Box::new(
                LocalVariableNestedForAttributeBindingAst::DestructureArray,
            ));
        let p2 = self
            .parse_local_variable_destructure_tuple()
            .enum_wrapper(Box::new(
                LocalVariableNestedForAttributeBindingAst::DestructureTuple,
            ));
        let p3 = self
            .parse_local_variable_destructure_object()
            .enum_wrapper(Box::new(
                LocalVariableNestedForAttributeBindingAst::DestructureObject,
            ));
        let p4 = self
            .parse_local_variable_single_identifier()
            .enum_wrapper(Box::new(
                LocalVariableNestedForAttributeBindingAst::SingleIdentifier,
            ));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_assignment_statement(&mut self) -> ParserResult<AssignmentStatementAst> {
        let c1 = self.current_pos();
        let p1 = self
            .parse_expression()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self
            .parse_expression()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        return Ok(AssignmentStatementAst::new(c1, p1, p2, p3));
    }

    fn parse_case_expression_branch_simple(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_else_case();
        let p2 = self.parse_pattern_statement_flavour_else();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    fn parse_case_expression_branch(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_destructuring();
        let p2 = self.parse_pattern_statement_flavour_non_destructuring();
        let p3 = self.parse_pattern_statement_flavour_else_case();
        let p4 = self.parse_pattern_statement_flavour_else();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_pattern_statement_flavour_destructuring(
        &mut self,
    ) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Is).parse_once()?;
        let p2 = self
            .parse_pattern_group_destructure()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_pattern_guard().parse_optional();
        let p4 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, p3, p4));
    }

    fn parse_pattern_statement_flavour_non_destructuring(
        &mut self,
    ) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_boolean_comparison_op().parse_once()?;
        let p2 = self
            .parse_pattern_variant_expression()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, None, p3));
    }

    fn parse_pattern_statement_flavour_else_case(
        &mut self,
    ) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else_case().parse_once()?;
        return Ok(CaseExpressionBranchAst::new_from_else_to_else_case(c1, p1));
    }

    fn parse_pattern_statement_flavour_else(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else().parse_once()?;
        let p2 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, None, vec![p1], None, p2));
    }

    fn parse_pattern_group_destructure(&mut self) -> ParserResult<PatternVariantAst> {
        let p1 = self
            .parse_pattern_variant_destructure_array()
            .enum_wrapper(Box::new(PatternVariantAst::DestructureArray));
        let p2 = self
            .parse_pattern_variant_destructure_tuple()
            .enum_wrapper(Box::new(PatternVariantAst::DestructureTuple));
        let p3 = self
            .parse_pattern_variant_destructure_object()
            .enum_wrapper(Box::new(PatternVariantAst::DestructureObject));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_pattern_variant_skip_argument(
        &mut self,
    ) -> ParserResult<PatternVariantDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_underscore().parse_once()?;
        return Ok(PatternVariantDestructureSkip1ArgumentAst::new(c1, p1));
    }

    fn parse_pattern_variant_skip_arguments(
        &mut self,
    ) -> ParserResult<PatternVariantDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self
            .parse_pattern_variant_single_identifier()
            .parse_optional();
        return Ok(PatternVariantDestructureSkipNArgumentsAst::new(c1, p1, p2));
    }

    fn parse_pattern_variant_single_identifier(
        &mut self,
    ) -> ParserResult<PatternVariantSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self
            .parse_local_variable_single_identifier_alias()
            .parse_optional();
        return Ok(PatternVariantSingleIdentifierAst::new(c1, p1, p2, p3));
    }

    fn parse_pattern_variant_destructure_tuple(
        &mut self,
    ) -> ParserResult<PatternVariantDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self
            .parse_pattern_variant_nested_for_destructure_tuple()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(PatternVariantDestructureTupleAst::new(c1, p1, p2, p3));
    }

    fn parse_pattern_variant_destructure_array(
        &mut self,
    ) -> ParserResult<PatternVariantDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self
            .parse_pattern_variant_nested_for_destructure_array()
            .parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(PatternVariantDestructureArrayAst::new(c1, p1, p2, p3));
    }

    fn parse_pattern_variant_destructure_object(
        &mut self,
    ) -> ParserResult<PatternVariantDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token_left_parenthesis().parse_once()?;
        let p3 = self
            .parse_pattern_variant_nested_for_destructure_object()
            .parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(PatternVariantDestructureObjectAst::new(c1, p1, p2, p3, p4));
    }

    fn parse_pattern_variant_attribute_binding(
        &mut self,
    ) -> ParserResult<PatternVariantAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self
            .parse_pattern_variant_nested_for_attribute_binding()
            .parse_once()?;
        return Ok(PatternVariantAttributeBindingAst::new(c1, p1, p2, p3));
    }

    fn parse_pattern_variant_literal(&mut self) -> ParserResult<PatternVariantLiteralAst> {
        let p1 = self
            .parse_literal_float()
            .enum_wrapper(Box::new(PatternVariantLiteralAst::Float));
        let p2 = self
            .parse_literal_integer()
            .enum_wrapper(Box::new(PatternVariantLiteralAst::Integer));
        let p3 = self
            .parse_literal_string()
            .enum_wrapper(Box::new(PatternVariantLiteralAst::String));
        let p4 = self
            .parse_literal_boolean()
            .enum_wrapper(Box::new(PatternVariantLiteralAst::Boolean));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_pattern_variant_expression(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_once()?;
        return Ok(PatternVariantAst::Expression(
            PatternVariantExpressionAst::new(c1, p1),
        ));
    }

    fn parse_pattern_variant_else(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Else).parse_once()?;
        return Ok(PatternVariantAst::Else(PatternVariantElseAst::new(c1, p1)));
    }

    fn parse_pattern_variant_else_case(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Else).parse_once()?;
        let p2 = self.parse_case_expression().parse_once()?;
        return Ok(PatternVariantAst::ElseCase(PatternVariantElseCaseAst::new(
            c1, p1, p2,
        )));
    }

    fn parse_pattern_variant_nested_for_destructure_array(
        &mut self,
    ) -> ParserResult<PatternVariantNestedForDestructureArrayAst> {
        let p1 = self
            .parse_pattern_variant_destructure_array()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::DestructureArray,
            ));
        let p2 = self
            .parse_pattern_variant_destructure_tuple()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::DestructureTuple,
            ));
        let p3 = self
            .parse_pattern_variant_destructure_object()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::DestructureObject,
            ));
        let p4 = self
            .parse_pattern_variant_single_identifier()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::SingleIdentifier,
            ));
        let p5 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(
            PatternVariantNestedForDestructureArrayAst::Literal,
        ));
        let p6 = self
            .parse_pattern_variant_skip_arguments()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::SkipNArgs,
            ));
        let p7 = self
            .parse_pattern_variant_skip_argument()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureArrayAst::Skip1Args,
            ));
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(p8);
    }

    fn parse_pattern_variant_nested_for_destructure_tuple(
        &mut self,
    ) -> ParserResult<PatternVariantNestedForDestructureTupleAst> {
        let p1 = self
            .parse_pattern_variant_destructure_array()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::DestructureArray,
            ));
        let p2 = self
            .parse_pattern_variant_destructure_tuple()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::DestructureTuple,
            ));
        let p3 = self
            .parse_pattern_variant_destructure_object()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::DestructureObject,
            ));
        let p4 = self
            .parse_pattern_variant_single_identifier()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::SingleIdentifier,
            ));
        let p5 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(
            PatternVariantNestedForDestructureTupleAst::Literal,
        ));
        let p6 = self
            .parse_pattern_variant_skip_arguments()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::SkipNArgs,
            ));
        let p7 = self
            .parse_pattern_variant_skip_argument()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureTupleAst::Skip1Args,
            ));
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(p8);
    }

    fn parse_pattern_variant_nested_for_destructure_object(
        &mut self,
    ) -> ParserResult<PatternVariantNestedForDestructureObjectAst> {
        let p1 = self
            .parse_pattern_variant_attribute_binding()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureObjectAst::AttrBind,
            ));
        let p2 = self
            .parse_pattern_variant_single_identifier()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureObjectAst::SingleIdentifier,
            ));
        let p3 = self
            .parse_pattern_variant_skip_arguments()
            .enum_wrapper(Box::new(
                PatternVariantNestedForDestructureObjectAst::SkipNArgs,
            ));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_pattern_variant_nested_for_attribute_binding(
        &mut self,
    ) -> ParserResult<PatternVariantNestedForAttributeBindingAst> {
        let p1 = self
            .parse_pattern_variant_destructure_array()
            .enum_wrapper(Box::new(
                PatternVariantNestedForAttributeBindingAst::DestructureArray,
            ));
        let p2 = self
            .parse_pattern_variant_destructure_tuple()
            .enum_wrapper(Box::new(
                PatternVariantNestedForAttributeBindingAst::DestructureTuple,
            ));
        let p3 = self
            .parse_pattern_variant_destructure_object()
            .enum_wrapper(Box::new(
                PatternVariantNestedForAttributeBindingAst::DestructureObject,
            ));
        let p4 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(
            PatternVariantNestedForAttributeBindingAst::Literal,
        ));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_pattern_guard(&mut self) -> ParserResult<PatternGuardAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::And).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        return Ok(PatternGuardAst::new(c1, p1, p2));
    }

    fn parse_binary_op_precedence_level_1(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_keyword(Keywords::Or).parse_once()?;
        return Ok(p1);
    }

    fn parse_binary_op_precedence_level_2(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_keyword(Keywords::And).parse_once()?;
        return Ok(p1);
    }

    fn parse_binary_op_precedence_level_3(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_keyword(Keywords::Is).parse_once()?;
        return Ok(p1);
    }

    fn parse_binary_op_precedence_level_4(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_eq();
        let p2 = self.parse_token_ne();
        let p3 = self.parse_token_le();
        let p4 = self.parse_token_ge();
        let p5 = self.parse_token_lt();
        let p6 = self.parse_token_gt();
        let p7 = self.parse_token_ss();
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(p8);
    }

    fn parse_binary_op_precedence_level_5(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_add_assign();
        let p2 = self.parse_token_sub_assign();
        let p3 = self.parse_token_add();
        let p4 = self.parse_token_sub();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_binary_op_precedence_level_6(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_mul_assign();
        let p2 = self.parse_token_div_assign();
        let p3 = self.parse_token_rem_assign();
        let p4 = self.parse_token_mod_assign();
        let p5 = self.parse_token_exp_assign();
        let p6 = self.parse_token_mul();
        let p7 = self.parse_token_div();
        let p8 = self.parse_token_rem();
        let p9 = self.parse_token_mod();
        let p10 = self.parse_token_exp();
        let p11 = p1
            .or(p2)
            .or(p3)
            .or(p4)
            .or(p5)
            .or(p6)
            .or(p7)
            .or(p8)
            .or(p9)
            .or(p10)
            .parse_once()?;
        return Ok(p11);
    }

    fn parse_boolean_comparison_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_eq();
        let p2 = self.parse_token_ne();
        let p3 = self.parse_token_le();
        let p4 = self.parse_token_ge();
        let p5 = self.parse_token_lt();
        let p6 = self.parse_token_gt();
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p8);
    }

    fn parse_unary_op(&mut self) -> ParserResult<UnaryExpressionOperatorAst> {
        let p1 = self.parse_unary_op_async_call().parse_once()?;
        return Ok(p1);
    }

    fn parse_unary_op_async_call(&mut self) -> ParserResult<UnaryExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Async).parse_once()?;
        return Ok(UnaryExpressionOperatorAst::Async(
            UnaryExpressionOperatorAsyncAst::new(c1, p1),
        ));
    }

    fn parse_postfix_op(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let p1 = self.parse_postfix_op_function_call();
        let p2 = self.parse_postfix_op_member_access();
        let p3 = self.parse_postfix_op_early_return();
        let p4 = self.parse_postfix_op_not_keyword();
        let p5 = self.parse_postfix_op_step_keyword();
        let p6 = p1.or(p2).or(p3).or(p4).or(p5).parse_once()?;
        return Ok(p6);
    }

    fn parse_postfix_op_function_call(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_generic_arguments().parse_optional();
        let p2 = self.parse_function_call_arguments().parse_once()?;
        let p3 = self.parse_token_double_dot().parse_optional();
        return Ok(PostfixExpressionOperatorAst::FunctionCall(
            PostfixExpressionOperatorFunctionCallAst::new(c1, p1, p2, p3),
        ));
    }

    fn parse_postfix_op_member_access(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let p1 = self.parse_postfix_op_member_access_runtime();
        let p2 = self.parse_postfix_op_member_access_static();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    fn parse_postfix_op_member_access_runtime(
        &mut self,
    ) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_dot().parse_once()?;
        let p2 = self.parse_identifier();
        let p3 = self.parse_numeric_identifier();
        let p4 = p2.or(p3).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::MemberAccess(
            PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p4),
        ));
    }

    fn parse_postfix_op_member_access_static(
        &mut self,
    ) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_colon().parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(PostfixExpressionOperatorAst::MemberAccess(
            PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2),
        ));
    }

    fn parse_postfix_op_early_return(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_question_mark().parse_once()?;
        return Ok(PostfixExpressionOperatorAst::EarlyReturn(
            PostfixExpressionOperatorEarlyReturnAst::new(c1, p1),
        ));
    }

    fn parse_postfix_op_not_keyword(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_dot().parse_once()?;
        let p2 = self.parse_keyword(Keywords::Not).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::NotKeyword(
            PostfixExpressionOperatorNotKeywordAst::new(c1, p1, p2),
        ));
    }

    fn parse_postfix_op_step_keyword(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_keyword(Keywords::Step).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::StepKeyword(
            PostfixExpressionOperatorStepKeywordAst::new(c1, p1, p2),
        ));
    }

    fn parse_convention(&mut self) -> ParserResult<ConventionAst> {
        let p1 = self.parse_convention_mut();
        let p2 = self.parse_convention_ref();
        let p3 = self.parse_convention_mov();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_convention_mov(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        return Ok(ConventionAst::new_mov(c1));
    }

    fn parse_convention_mut(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_borrow().parse_once()?;
        let p2 = self.parse_keyword(Keywords::Mut).parse_once()?;
        return Ok(ConventionAst::new_mut(c1, p1, p2));
    }

    fn parse_convention_ref(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_borrow().parse_once()?;
        return Ok(ConventionAst::new_ref(c1, p1));
    }

    fn parse_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_object_initializer_arguments().parse_once()?;
        return Ok(ObjectInitializerAst::new(c1, p1, p2));
    }

    fn parse_object_initializer_arguments(
        &mut self,
    ) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self
            .parse_object_initializer_argument()
            .parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3));
    }

    fn parse_object_initializer_argument(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let p1 = self.parse_object_initializer_argument_named();
        let p2 = self.parse_object_initializer_argument_unnamed();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    fn parse_object_initializer_argument_unnamed(
        &mut self,
    ) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_unnamed(c1, p1, p2));
    }

    fn parse_object_initializer_argument_named(
        &mut self,
    ) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_named(
            c1,
            p1,
            p2,
            Box::new(p3),
        ));
    }

    fn parse_type(&mut self) -> ParserResult<TypeAst> {
        let p1 = self.parse_type_optional();
        let p2 = self.parse_type_variant();
        let p3 = self.parse_type_tuple();
        let p4 = self.parse_type_single();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    fn parse_type_optional(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_question_mark().parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        return Ok(TypeOptionalAst::new(c1, p1, p2).to_type());
    }

    fn parse_type_single(&mut self) -> ParserResult<TypeAst> {
        let p1 = self.parse_type_single_with_namespace();
        let p2 = self.parse_type_single_with_self();
        let p3 = self.parse_type_single_without_namespace_or_self();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_type_single_with_namespace(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self
            .parse_identifier()
            .parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        let p2 = self.parse_token_double_colon().parse_once()?;
        let p3 = self
            .parse_generic_identifier()
            .parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        return Ok(TypeAst::new(c1, p1, p3));
    }

    fn parse_type_single_with_self(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_self_type_keyword().parse_once()?;
        let p2 = self
            .parse_types_after_self()
            .parse_optional()
            .unwrap_or_default();
        return Ok(TypeAst::new(
            c1,
            vec![],
            vec![p1].into_iter().chain(p2.into_iter()).collect(),
        ));
    }

    fn parse_types_after_self(&mut self) -> ParserResult<Vec<GenericIdentifierAst>> {
        let p1 = self.parse_token_double_colon().parse_once()?;
        let p2 = self
            .parse_generic_identifier()
            .parse_zero_or_more(Box::new(self.parse_token_double_colon()));
        return Ok(p2);
    }

    fn parse_type_single_without_namespace_or_self(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self
            .parse_generic_identifier()
            .parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        return Ok(TypeAst::new(c1, vec![], p1));
    }

    fn parse_self_type_keyword(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::SelfType).parse_once()?;
        return Ok(GenericIdentifierAst::new(c1, "Self".to_string(), None));
    }

    fn parse_type_tuple(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self
            .parse_type()
            .parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(TypeTupleAst::new(c1, p1, p2, p3).to_type());
    }

    fn parse_type_non_union(&mut self) -> ParserResult<TypeAst> {
        let p1 = self.parse_type_single();
        let p2 = self.parse_type_tuple();
        let p3 = self.parse_type_optional();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_type_variant(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self
            .parse_type_non_union()
            .parse_two_or_more(Box::new(self.parse_token_union()))?;
        return Ok(TypeVariantAst::new(c1, p1).to_type());
    }

    fn parse_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_identifier().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    fn parse_numeric_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_dec_integer().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    fn parse_upper_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_upper_identifier().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    fn parse_generic_identifier(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_arguments().parse_optional();
        return Ok(GenericIdentifierAst::new(c1, p1.value, p2));
    }

    fn parse_literal(&mut self) -> ParserResult<LiteralAst> {
        let p1 = self.parse_literal_float();
        let p2 = self.parse_literal_integer();
        let p3 = self.parse_literal_string();
        let p4 = self.parse_literal_tuple(Rc::new(RefCell::new(|| self.parse_expression())));
        let p5 = self.parse_literal_array(Rc::new(RefCell::new(|| self.parse_expression())));
        let p6 = self.parse_literal_boolean();
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    fn parse_literal_float(&mut self) -> ParserResult<LiteralAst> {
        let p1 = self.parse_literal_float_b10().parse_once()?;
        return Ok(p1);
    }

    fn parse_literal_integer(&mut self) -> ParserResult<LiteralAst> {
        let p1 = self.parse_literal_integer_b10();
        let p2 = self.parse_literal_integer_b02();
        let p3 = self.parse_literal_integer_b16();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_literal_string(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_string().parse_once()?;
        return Ok(LiteralAst::new_string(c1, p1));
    }

    fn parse_literal_tuple(
        &mut self,
        item: Rc<RefCell<dyn FnMut() -> ParserResult<ExpressionAst>>>,
    ) -> ParserResult<LiteralAst> {
        let p1 = self.parse_literal_tuple_0_items();
        let p2 = self.parse_literal_tuple_1_items(item.clone());
        let p3 = self.parse_literal_tuple_n_items(item.clone());
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    fn parse_literal_array(
        &mut self,
        item: Rc<RefCell<dyn FnMut() -> ParserResult<ExpressionAst>>>,
    ) -> ParserResult<LiteralAst> {
        let p1 = self.parse_literal_array_0_items();
        let p2 = self.parse_literal_array_n_items(item.clone());
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    fn parse_literal_boolean(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::True);
        let p2 = self.parse_keyword(Keywords::False);
        let p3 = p1.or(p2).parse_once()?;
        return Ok(LiteralAst::new_boolean(c1, p3));
    }

    fn parse_literal_float_b10(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_dec_integer().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p4 = self.parse_lexeme_dec_integer().parse_once()?;
        let p5 = self.parse_float_postfix_type().parse_optional();
        return Ok(LiteralAst::new_float(
            c1,
            p1,
            p2,
            p3,
            p4,
            p5.and_then(|x| Some(TypeAst::from(x))),
        ));
    }

    fn parse_literal_integer_b10(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_dec_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(
            c1,
            p1,
            p2,
            p3.and_then(|x| Some(TypeAst::from(x))),
        ));
    }

    fn parse_literal_integer_b02(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_bin_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(
            c1,
            p1,
            p2,
            p3.and_then(|x| Some(TypeAst::from(x))),
        ));
    }

    fn parse_literal_integer_b16(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_hex_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(
            c1,
            p1,
            p2,
            p3.and_then(|x| Some(TypeAst::from(x))),
        ));
    }

    fn parse_numeric_prefix_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_sub();
        let p2 = self.parse_token_add();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    fn parse_integer_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkUnderscore)
            .parse_once()?;
        let p2 = self.parse_characters("i8");
        let p3 = self.parse_characters("i16");
        let p4 = self.parse_characters("i32");
        let p5 = self.parse_characters("i64");
        let p6 = self.parse_characters("i128");
        let p7 = self.parse_characters("i256");
        let p8 = self.parse_characters("u8");
        let p9 = self.parse_characters("u16");
        let p10 = self.parse_characters("u32");
        let p11 = self.parse_characters("u64");
        let p12 = self.parse_characters("u128");
        let p13 = self.parse_characters("u256");
        let p14 = p2
            .or(p3)
            .or(p4)
            .or(p5)
            .or(p6)
            .or(p7)
            .or(p8)
            .or(p9)
            .or(p10)
            .or(p11)
            .or(p12)
            .or(p13)
            .parse_once()?;
        return Ok(IdentifierAst::new(p14.pos, p14.metadata));
    }

    fn parse_float_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkUnderscore)
            .parse_once()?;
        let p2 = self.parse_characters("f8");
        let p3 = self.parse_characters("f16");
        let p4 = self.parse_characters("f32");
        let p5 = self.parse_characters("f64");
        let p6 = self.parse_characters("f128");
        let p7 = self.parse_characters("f256");
        let p8 = p2.or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(IdentifierAst::new(p8.pos, p8.metadata));
    }

    fn parse_literal_tuple_0_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, vec![], p2));
    }

    fn parse_literal_tuple_1_items(
        &mut self,
        item: Rc<RefCell<dyn FnMut<(), Output = ParserResult<ExpressionAst>>>>,
    ) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = item.borrow_mut()().parse_once()?;
        let p3 = self.parse_token_comma().parse_once()?;
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, vec![p2], p4));
    }

    fn parse_literal_tuple_n_items(
        &mut self,
        item: Rc<RefCell<dyn FnMut<(), Output = ParserResult<ExpressionAst>>>>,
    ) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = item.borrow_mut()().parse_two_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, p2, p3));
    }

    fn parse_literal_array_0_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        let p3 = self
            .parse_token_primitive(TokenType::TkComma)
            .parse_once()?;
        let p4 = self.parse_lexeme_dec_integer().parse_once()?;
        let p5 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(LiteralAst::new_array_0(c1, p1, p2, p3, p4, p5));
    }

    fn parse_literal_array_n_items(
        &mut self,
        item: Rc<RefCell<dyn FnMut() -> ParserResult<ExpressionAst>>>,
    ) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = item.borrow_mut()().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(LiteralAst::new_array_n(c1, p1, p2, p3));
    }

    fn parse_global_constant_value(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = self
            .parse_literal_float()
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p2 = self
            .parse_literal_integer()
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p3 = self
            .parse_literal_string()
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p4 = self
            .parse_literal_tuple(Rc::new(RefCell::new(|| self.parse_global_constant_value())))
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p5 = self
            .parse_literal_array(Rc::new(RefCell::new(|| self.parse_global_constant_value())))
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p6 = self
            .parse_literal_boolean()
            .enum_wrapper(Box::new(PrimaryExpressionAst::Literal))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p7 = self
            .parse_global_object_initializer()
            .enum_wrapper(Box::new(PrimaryExpressionAst::ObjectInitializer))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p8 = self
            .parse_identifier()
            .enum_wrapper(Box::new(PrimaryExpressionAst::Identifier))
            .enum_wrapper(Box::new(ExpressionAst::Primary));
        let p9 = p1
            .or(p2)
            .or(p3)
            .or(p4)
            .or(p5)
            .or(p6)
            .or(p7)
            .or(p8)
            .parse_once()?;
        return Ok(p9);
    }

    fn parse_global_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self
            .parse_global_object_initializer_arguments()
            .parse_once()?;
        return Ok(ObjectInitializerAst::new(c1, p1, p2));
    }

    fn parse_global_object_initializer_arguments(
        &mut self,
    ) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self
            .parse_global_object_initializer_argument_named()
            .parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3));
    }

    fn parse_global_object_initializer_argument_named(
        &mut self,
    ) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_global_constant_value().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_named(
            c1,
            p1,
            p2,
            Box::new(p3),
        ));
    }

    fn parse_keyword(&mut self, keyword: Keywords) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        for c in keyword.to_string().chars() {
            let p1 = self.parse_character(c);
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, keyword.to_string()));
    }

    fn parse_token_left_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLeftCurlyBrace)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_right_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkRightCurlyBrace)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_left_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLeftParenthesis)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_right_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkRightParenthesis)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_left_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLeftSquareBracket)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_right_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkRightSquareBracket)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        return Ok(p1);
    }

    fn parse_token_double_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkColon)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkColon)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_comma(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkComma)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkColon)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_newline(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkNewLine)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_rightward_arrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkMinusSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkGreaterThanSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_double_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        return Ok(p1);
    }

    fn parse_token_question_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkQuestionMark)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_borrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkAmpersand)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_union(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkVerticalBar)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_eq(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_ne(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkExclamationMark)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_le(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLessThanSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_ge(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkGreaterThanSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_lt(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLessThanSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_gt(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkGreaterThanSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_ss(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkLessThanSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        let p3 = self
            .parse_token_primitive(TokenType::TkGreaterThanSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_add_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPlusSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_sub_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkMinusSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_add(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPlusSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_sub(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkMinusSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_mul_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_div_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkForwardSlash)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_rem_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_mod_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        let p3 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_exp_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        let p3 = self
            .parse_token_primitive(TokenType::TkEqualsSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_mul(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_div(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkForwardSlash)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_rem(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_mod(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkPercentSign)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_exp(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        let p2 = self
            .parse_token_primitive(TokenType::TkAsterisk)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_underscore(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkUnderscore)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_speech_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkSpeechMark)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_no_token(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        return Ok(TokenAst::new(c1, TokenType::NoToken, "".to_string()));
    }

    fn parse_lexeme_dec_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkNumber(num))
                .parse_once()?;
            integer.push(char::from(p1.metadata.as_bytes()[0]));
        }
        if integer.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected decimal integer".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, integer));
    }

    fn parse_lexeme_bin_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        self.parse_token_primitive(TokenType::TkNumber('0'))
            .parse_once()?;
        self.parse_token_primitive(TokenType::TkCharacter('b'))
            .parse_once()?;
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkNumber(num))
                .parse_once()?;
            integer.push(char::from(p1.metadata.as_bytes()[0]));
            if num.is_digit(2) {
                return Err(SyntaxError::new(c1, "Expected binary integer".to_string()));
            }
        }
        if integer.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected binary integer".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, integer));
    }

    fn parse_lexeme_hex_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        self.parse_token_primitive(TokenType::TkNumber('0'))
            .parse_once()?;
        self.parse_token_primitive(TokenType::TkCharacter('x'))
            .parse_once()?;
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkNumber(num))
                .parse_once()?;
            integer.push(char::from(p1.metadata.as_bytes()[0]));
            if !num.is_digit(16) {
                return Err(SyntaxError::new(
                    c1,
                    "Expected hexadecimal integer".to_string(),
                ));
            }
        }
        if integer.len() == 0 {
            return Err(SyntaxError::new(
                c1,
                "Expected hexadecimal integer".to_string(),
            ));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, integer));
    }

    fn parse_lexeme_string(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        self.parse_token_speech_mark().parse_once()?;
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkCharacter(string))
                .parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        self.parse_token_speech_mark().parse_once()?;
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    fn parse_lexeme_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkCharacter(string))
                .parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected identifier".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    fn parse_lexeme_upper_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self
                .parse_token_primitive(TokenType::TkCharacter(string))
                .parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier.len() == 0 {
            return Err(SyntaxError::new(
                c1,
                "Expected upper identifier".to_string(),
            ));
        }
        if (identifier.as_bytes()[0] as char).is_lowercase() {
            return Err(SyntaxError::new(
                c1,
                "Expected upper identifier".to_string(),
            ));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    fn parse_characters(&mut self, characters: &str) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        for c in characters.chars() {
            let p1 = self.parse_character(c).parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier != characters {
            return Err(SyntaxError::new(
                c1,
                format!("Expected '{}', got '{}'", characters, identifier),
            ));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, "".to_string()));
    }

    fn parse_character(&mut self, character: char) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::TkCharacter(character))
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_eof(&mut self) -> ParserResult<TokenAst> {
        let p1 = self
            .parse_token_primitive(TokenType::EndOfFile)
            .parse_once()?;
        return Ok(p1);
    }

    fn parse_token_primitive(&mut self, token_type: TokenType) -> ParserResult<TokenAst> {
        if self.index > self.token_len {
            let new_error = format!("Expected token {:?} but found end of file", token_type);
            self.store_error(self.current_pos(), new_error);
            return Err(self.error.clone());
        }

        if token_type != TokenType::TkNewLine {
            while self.tokens[self.index] == TokenType::TkNewLine {
                self.index += 1;
            }
        }

        if self.tokens[self.index] != token_type {
            if self.error.pos == self.index {
                self.error.expected_tokens.insert(token_type);
                return Err(self.error.clone());
            }

            let new_error = format!("Expected £, got '{:?}'", self.tokens[self.index]);
            if self.store_error(self.index, new_error) {
                self.error.expected_tokens.insert(token_type);
            }
            return Err(self.error.clone());
        }

        let r = TokenAst::new(
            self.current_pos(),
            self.tokens[self.index].clone(),
            "".to_string(),
        );
        self.index += 1;
        return Ok(r);
    }
}
