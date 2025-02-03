use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::assignment_statement_ast::AssignmentStatementAst;
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
use crate::asts::local_variable_ast::{LocalVariableAst, LocalVariableNestedForAttributeBindingAst, LocalVariableNestedForDestructureArrayAst, LocalVariableNestedForDestructureObjectAst, LocalVariableNestedForDestructureTupleAst};
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
use crate::asts::pattern_variant_ast::{PatternVariantAst, PatternVariantNestedForAttributeBindingAst, PatternVariantNestedForDestructureArrayAst, PatternVariantNestedForDestructureObjectAst, PatternVariantNestedForDestructureTupleAst};
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
use crate::parser::parser_rule_handler::{ParserRuleHandler, SingleParserRuleHandler};
use SPP_Compiler_Rust::parser_rule;
use std::rc::Rc;
use std::cell::RefCell;
use crate::asts::ast::ToBinaryExpression;

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
        }
        else {
            false
        }
    }
}


impl Parser {
    #[parser_rule]
    fn parse_root(self: &'static Rc<Self>) -> SingleParserRuleHandler<ModulePrototypeAst> {
        let p1 = self.parse_module_prototype().parse_once()?;
        let p2 = self.parse_eof().parse_once()?;
        Ok(p1)
    }

    #[parser_rule]
    fn parse_module_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<ModulePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_module_implementation().parse_once()?;
        Ok(ModulePrototypeAst::new(c1, p1))
    }

    #[parser_rule]
    fn parse_module_implementation(self: &'static Rc<Self>) -> SingleParserRuleHandler<ModuleImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_module_member().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        Ok(ModuleImplementationAst::new(c1, p1))
    }

    #[parser_rule]
    fn parse_module_member(self: &'static Rc<Self>) -> SingleParserRuleHandler<ModuleMemberAst> {
        let p1 = self.parse_function_prototype().enum_wrapper(Box::new(ModuleMemberAst::Function));
        let p2 = self.parse_class_prototype().enum_wrapper(Box::new(ModuleMemberAst::Class));
        let p3 = self.parse_sup_prototype_extension().enum_wrapper(Box::new(ModuleMemberAst::SupExtension));
        let p4 = self.parse_sup_prototype_functions().enum_wrapper(Box::new(ModuleMemberAst::SupFunctions));
        let p5 = self.parse_global_use_statement().enum_wrapper(Box::new(ModuleMemberAst::UseStatement));
        let p6 = self.parse_global_constant().enum_wrapper(Box::new(ModuleMemberAst::GlobalConst));
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    #[parser_rule]
    fn parse_class_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<ClassPrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let p2 = self.parse_keyword(Keywords::Cls).parse_once()?;
        let p3 = self.parse_upper_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_where_block().parse_optional();
        let p6 = self.parse_class_implementation().parse_once()?;
        return Ok(ClassPrototypeAst::new(c1, p1, p2, TypeAst::from(p3), p4, p5, p6));
    }

    #[parser_rule]
    fn parse_class_implementation(self: &'static Rc<Self>) -> SingleParserRuleHandler<ClassImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self.parse_class_member().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return Ok(ClassImplementationAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_class_member(self: &'static Rc<Self>) -> SingleParserRuleHandler<ClassMemberAst> {
        let p1 = self.parse_class_attribute().parse_once()?;
        return Ok(ClassMemberAst::Attr(p1));
    }

    #[parser_rule]
    fn parse_class_attribute(self: &'static Rc<Self>) -> SingleParserRuleHandler<ClassAttributeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token_colon().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return Ok(ClassAttributeAst::new(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_sup_prototype_functions(self: &'static Rc<Self>) -> SingleParserRuleHandler<SupPrototypeFunctionsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Sup).parse_once()?;
        let p2 = self.parse_generic_parameters().parse_optional();
        let p3 = self.parse_type().parse_once()?;
        let p4 = self.parse_where_block().parse_optional();
        let p5 = self.parse_sup_implementation().parse_once()?;
        return Ok(SupPrototypeFunctionsAst::new(c1, p1, p2, p3, p4, p5));
    }

    #[parser_rule]
    fn parse_sup_prototype_extension(self: &'static Rc<Self>) -> SingleParserRuleHandler<SupPrototypeExtensionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Sup).parse_once()?;
        let p2 = self.parse_generic_parameters().parse_optional();
        let p3 = self.parse_type().parse_once()?;
        let p4 = self.parse_keyword(Keywords::Ext).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        let p6 = self.parse_where_block().parse_optional();
        let p7 = self.parse_sup_implementation().parse_once()?;
        return Ok(SupPrototypeExtensionAst::new(c1, p1, p2, p3, p4, p5, p6, p7));
    }

    #[parser_rule]
    fn parse_sup_implementation(self: &'static Rc<Self>) -> SingleParserRuleHandler<SupImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self.parse_sup_member().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return Ok(SupImplementationAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_sup_member(self: &'static Rc<Self>) -> SingleParserRuleHandler<SupMemberAst> {
        let p1 = self.parse_sup_method_prototype().enum_wrapper(Box::new(SupMemberAst::Method));
        let p2 = self.parse_sup_use_statement().enum_wrapper(Box::new(SupMemberAst::Typedef));
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_sup_method_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionPrototypeAst> {
        let p1 = self.parse_function_prototype().parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_sup_use_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<SupUseStatementAst> {
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let mut p2 = self.parse_use_statement().parse_once()?;
        p2.annotations = p1;
        return Ok(p2);
    }

    #[parser_rule]
    fn parse_function_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionPrototypeAst> {
        let p1 = self.parse_subroutine_prototype().enum_wrapper(Box::new(FunctionPrototypeAst::Subroutine));
        let p2 = self.parse_coroutine_prototype().enum_wrapper(Box::new(FunctionPrototypeAst::Coroutine));
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_subroutine_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<SubroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let p2 = self.parse_keyword(Keywords::Fun).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_function_parameters().parse_once()?;
        let p6 = self.parse_token_rightward_arrow().parse_once()?;
        let p7 = self.parse_type().parse_once()?;
        let p8 = self.parse_where_block().parse_optional();
        let p9 = self.parse_function_implementation().parse_once()?;
        return Ok(SubroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9)));
    }

    #[parser_rule]
    fn parse_coroutine_prototype(self: &'static Rc<Self>) -> SingleParserRuleHandler<CoroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let p2 = self.parse_keyword(Keywords::Cor).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_function_parameters().parse_once()?;
        let p6 = self.parse_token_rightward_arrow().parse_once()?;
        let p7 = self.parse_type().parse_once()?;
        let p8 = self.parse_where_block().parse_optional();
        let p9 = self.parse_function_implementation().parse_once()?;
        return Ok(CoroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9)));
    }

    #[parser_rule]
    fn parse_function_implementation(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkLeftCurlyBrace).parse_once()?;
        let p2 = self.parse_function_member().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        let p3 = self.parse_token_primitive(TokenType::TkRightCurlyBrace).parse_once()?;
        return Ok(FunctionImplementationAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_function_member(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionMemberAst> {
        let p1 = self.parse_statement().parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_function_call_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionCallArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_function_call_argument().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(FunctionCallArgumentGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_function_call_argument(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionCallArgumentAst> {
        let p1 = self.parse_function_call_argument_named();
        let p2 = self.parse_function_call_argument_unnamed();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_function_call_argument_unnamed(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_convention().parse_once()?;
        let p2 = self.parse_token_double_dot().parse_optional();
        let p3 = self.parse_expression().parse_once()?;
        return Ok(FunctionCallArgumentAst::new_unnamed_argument(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_function_call_argument_named(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_expression().parse_once()?;
        return Ok(FunctionCallArgumentAst::new_named_argument(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_function_parameters(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_function_parameter().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(FunctionParameterGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_function_parameter(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterAst> {
        let p1 = self.parse_function_parameter_variadic();
        let p2 = self.parse_function_parameter_optional();
        let p3 = self.parse_function_parameter_required();
        let p4 = self.parse_function_parameter_self();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_function_parameter_self(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_convention().parse_once()?;
        let p3 = self.parse_self_keyword().parse_once()?;
        return Ok(FunctionParameterAst::new_self(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_function_parameter_required(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return Ok(FunctionParameterAst::new_required(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_function_parameter_optional(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        let p5 = self.parse_token_assign().parse_once()?;
        let p6 = self.parse_expression().parse_once()?;
        return Ok(FunctionParameterAst::new_optional(c1, p1, p2, p3, p4, p5, p6));
    }

    #[parser_rule]
    fn parse_function_parameter_variadic(self: &'static Rc<Self>) -> SingleParserRuleHandler<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_convention().parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return Ok(FunctionParameterAst::new_variadic(c1, p1, p2, p3, p4, p5));
    }

    #[parser_rule]
    fn parse_generic_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_generic_argument().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(GenericArgumentGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_generic_argument(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentAst> {
        let p1 = self.parse_generic_type_argument_named();
        let p2 = self.parse_generic_type_argument_unnamed();
        let p3 = self.parse_generic_comp_argument_named();
        let p4 = self.parse_generic_comp_argument_unnamed();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_generic_type_argument_named(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_type().parse_once()?;
        return Ok(GenericArgumentAst::new_type_named(c1, TypeAst::from(p1), p2, p3));
    }

    #[parser_rule]
    fn parse_generic_type_argument_unnamed(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type().parse_once()?;
        return Ok(GenericArgumentAst::new_type_unnamed(c1, p1));
    }

    #[parser_rule]
    fn parse_generic_comp_argument_named(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_global_constant_value().parse_once()?;
        return Ok(GenericArgumentAst::new_comp_named(c1, TypeAst::from(p1), p2, p3));
    }

    #[parser_rule]
    fn parse_generic_comp_argument_unnamed(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_global_constant_value().parse_once()?;
        return Ok(GenericArgumentAst::new_comp_unnamed(c1, p1));
    }

    #[parser_rule]
    fn parse_generic_parameters(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_generic_parameter().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(GenericParameterGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_generic_parameter(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let p1 = self.parse_generic_comp_parameter_variadic();
        let p2 = self.parse_generic_comp_parameter_optional();
        let p3 = self.parse_generic_comp_parameter_required();
        let p4 = self.parse_generic_type_parameter_variadic();
        let p5 = self.parse_generic_type_parameter_optional();
        let p6 = self.parse_generic_type_parameter_required();
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    #[parser_rule]
    fn parse_generic_type_parameter_required(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_inline_constraints().parse_optional();
        return Ok(GenericParameterAst::new_type_required(c1, TypeAst::from(p1), p2));
    }

    #[parser_rule]
    fn parse_generic_type_parameter_optional(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_inline_constraints().parse_optional();
        let p3 = self.parse_token_assign().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return Ok(GenericParameterAst::new_type_optional(c1, TypeAst::from(p1), p2, p3, p4));
    }

    #[parser_rule]
    fn parse_generic_type_parameter_variadic(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_upper_identifier().parse_once()?;
        let p3 = self.parse_generic_inline_constraints().parse_optional();
        return Ok(GenericParameterAst::new_type_variadic(c1, p1, TypeAst::from(p2), p3));
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_required(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return Ok(GenericParameterAst::new_comp_required(c1, p1, TypeAst::from(p2), p3, p4));
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_optional(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        let p5 = self.parse_token_assign().parse_once()?;
        let p6 = self.parse_global_constant_value().parse_once()?;
        return Ok(GenericParameterAst::new_comp_optional(c1, p1, TypeAst::from(p2), p3, p4, p5, p6));
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_variadic(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_token_double_dot().parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return Ok(GenericParameterAst::new_comp_variadic(c1, p1, p2, TypeAst::from(p3), p4, p5));
    }

    #[parser_rule]
    fn parse_generic_inline_constraints(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericParameterConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p2 = self.parse_type().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        return Ok(GenericParameterConstraintsAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_where_block(self: &'static Rc<Self>) -> SingleParserRuleHandler<WhereBlockAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Where).parse_once()?;
        let p2 = self.parse_where_block_constraints_group().parse_once()?;
        return Ok(WhereBlockAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_where_block_constraints_group(self: &'static Rc<Self>) -> SingleParserRuleHandler<WhereConstraintsGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_where_block_constraints().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(WhereConstraintsGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_where_block_constraints(self: &'static Rc<Self>) -> SingleParserRuleHandler<WhereConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p2 = self.parse_token_colon().parse_once()?;
        let p3 = self.parse_type().parse_once()?;
        return Ok(WhereConstraintsAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_annotation(self: &'static Rc<Self>) -> SingleParserRuleHandler<AnnotationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkAt).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(AnnotationAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        let p1 = self.parse_binary_expression_precedence_level_1().parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_binary_expression_precedence_level_n_rhs<T: ToBinaryExpression + 'static>(self: &'static Rc<Self>, op: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<TokenAst>>>>, rhs: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<T>>>>) -> SingleParserRuleHandler<(TokenAst, T)> {
        let p1 = op.borrow_mut()().parse_once()?;
        let p2 = rhs.borrow_mut()().parse_once()?;
        return Ok((p1, p2));
    }

    #[parser_rule]
    fn parse_binary_expression_precedence_level_n<T: ToBinaryExpression + 'static>(self: &'static Rc<Self>, lhs: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<ExpressionAst>>>>, op: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<TokenAst>>>>, rhs: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<T>>>>) -> SingleParserRuleHandler<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = lhs.borrow_mut()().parse_once()?;
        let p2 = self.parse_binary_expression_precedence_level_n_rhs(op.clone(), rhs.clone()).parse_optional();
        return Ok(if let Some(p2) = p2 { T::to_binary_expression(c1, p1, p2.0, p2.1) } else { p1 });
    }

    fn parse_binary_expression_precedence_level_1(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_2())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_1())),
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_1())))
    }

    fn parse_binary_expression_precedence_level_2(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_3())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_2())),
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_2())))
    }

    fn parse_binary_expression_precedence_level_3(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_4())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_3())),
            Rc::new(RefCell::new(|| self.parse_pattern_group_destructure())))
    }

    fn parse_binary_expression_precedence_level_4(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_5())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_4())),
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_4())))
    }

    fn parse_binary_expression_precedence_level_5(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_6())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_5())),
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_5())))
    }

    fn parse_binary_expression_precedence_level_6(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Rc::new(RefCell::new(|| self.parse_unary_expression())),
            Rc::new(RefCell::new(|| self.parse_binary_op_precedence_level_6())),
            Rc::new(RefCell::new(|| self.parse_binary_expression_precedence_level_6())))
    }

    #[parser_rule]
    fn parse_unary_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_unary_op().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        let p2 = self.parse_postfix_expression().parse_once()?;
        return p1.into_iter().rev().fold(Ok(p2), |acc, x| Ok(ExpressionAst::Unary(UnaryExpressionAst::new(c1, x, Box::from(acc?)))));
    }

    #[parser_rule]
    fn parse_postfix_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_primary_expression().parse_once()?;
        let p2 = self.parse_postfix_op().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        return p2.into_iter().fold(Ok(p1), |acc, x| Ok(ExpressionAst::Postfix(PostfixExpressionAst::new(c1, Box::from(acc?), x))));
    }

    #[parser_rule]
    fn parse_primary_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        let p1 = self.parse_literal().enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p2 = self.parse_object_initializer().enum_wrapper(Box::new(PrimaryExpressionAst::ObjectInitializer)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p4 = self.parse_parenthesized_expression().enum_wrapper(Box::new(PrimaryExpressionAst::Parenthesized)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p5 = self.parse_type().enum_wrapper(Box::new(PrimaryExpressionAst::Type)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p6 = self.parse_identifier().enum_wrapper(Box::new(PrimaryExpressionAst::Identifier)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p7 = self.parse_case_expression().enum_wrapper(Box::new(PrimaryExpressionAst::Case)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p8 = self.parse_loop_expression().enum_wrapper(Box::new(PrimaryExpressionAst::Loop)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p9 = self.parse_gen_expression().enum_wrapper(Box::new(PrimaryExpressionAst::Gen)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p10 = self.parse_with_expression().enum_wrapper(Box::new(PrimaryExpressionAst::With)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p11 = self.parse_inner_scope().enum_wrapper(Box::new(PrimaryExpressionAst::InnerScope)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p12 = self.parse_self_keyword().enum_wrapper(Box::new(PrimaryExpressionAst::SelfIdentifier)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p13 = self.parse_token_double_dot().enum_wrapper(Box::new(PrimaryExpressionAst::Fold)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p14 = p1.or(p2).or(p4).or(p5).or(p6).or(p7).or(p8).or(p9).or(p10).or(p11).or(p12).or(p13).parse_once()?;
        return Ok(p14);
    }

    #[parser_rule]
    fn parse_parenthesized_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<ParenthesizedExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(ParenthesizedExpressionAst::new(c1, p1, Box::from(p2), p3));
    }

    #[parser_rule]
    fn parse_self_keyword(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::SelfVal).parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    #[parser_rule]
    fn parse_case_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionAst> {
        let p1 = self.parse_case_expression_patterns();
        let p2 = self.parse_case_expression_simple();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_case_expression_patterns(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Case).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_keyword(Keywords::Of).parse_once()?;
        let p4 = self.parse_case_expression_branch().parse_one_or_more(Box::new(self.parse_token_no_token()))?;
        return Ok(CaseExpressionAst::new(c1, p1, Box::new(p2), p3, p4));
    }

    #[parser_rule]
    fn parse_case_expression_simple(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Case).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_inner_scope().parse_once()?;
        let p4 = self.parse_case_expression_branch_simple().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        return Ok(CaseExpressionAst::new_from_simple(c1, p1, Box::new(p2), p3, p4));
    }

    #[parser_rule]
    fn parse_loop_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Loop).parse_once()?;
        let p2 = self.parse_loop_expression_condition().parse_once()?;
        let p3 = self.parse_inner_scope().parse_once()?;
        let p4 = self.parse_loop_else_statement().parse_optional();
        return Ok(LoopExpressionAst::new(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_loop_expression_condition(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopConditionAst> {
        let p1 = self.parse_loop_expression_condition_iterable();
        let p2 = self.parse_loop_expression_condition_boolean();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_loop_expression_condition_boolean(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_once()?;
        return Ok(LoopConditionAst::new_boolean(c1, Box::new(p1)));
    }

    #[parser_rule]
    fn parse_loop_expression_condition_iterable(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_keyword(Keywords::In).parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return Ok(LoopConditionAst::new_iterable(c1, p1, p2, Box::new(p3)));
    }

    #[parser_rule]
    fn parse_loop_else_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopElseStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Else).parse_once()?;
        let p2 = self.parse_inner_scope().parse_once()?;
        return Ok(LoopElseStatementAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_gen_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenExpressionAst> {
        let p1 = self.parse_gen_expression_unroll();
        let p2 = self.parse_gen_expression_normal();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_gen_expression_normal(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenExpressionAst> {
        let p1 = self.parse_gen_expression_normal_with_expression();
        let p2 = self.parse_gen_expression_normal_no_expression();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_gen_expression_normal_no_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Gen).parse_once()?;
        return Ok(GenExpressionAst::new(c1, p1, None, ConventionAst::Mov{pos: c1}, None));
    }

    #[parser_rule]
    fn parse_gen_expression_normal_with_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Gen).parse_once()?;
        let p2 = self.parse_convention().parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return Ok(GenExpressionAst::new(c1, p1, None, p2, Some(Box::new(p3))));
    }

    #[parser_rule]
    fn parse_gen_expression_unroll(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Gen).parse_once()?;
        let p2 = self.parse_keyword(Keywords::With).parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return Ok(GenExpressionAst::new(c1, p1, Some(p2), ConventionAst::Mov{pos: c1}, Some(Box::new(p3))));
    }

    #[parser_rule]
    fn parse_with_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<WithExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::With).parse_once()?;
        let p2 = self.parse_with_expression_lhs_alias().parse_optional();
        let p3 = self.parse_expression().parse_once()?;
        let p4 = self.parse_inner_scope().parse_once()?;
        return Ok(WithExpressionAst::new(c1, p1, p2, Box::new(p3), p4));
    }

    #[parser_rule]
    fn parse_with_expression_lhs_alias(self: &'static Rc<Self>) -> SingleParserRuleHandler<WithExpressionAliasAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        return Ok(WithExpressionAliasAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_ret_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<RetStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Ret).parse_once()?;
        let p2 = self.parse_expression().parse_optional();
        return Ok(RetStatementAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_exit_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Exit).parse_one_or_more(Box::new(self.parse_token_no_token()))?;
        let p2 = self.parse_exit_statement_final_action().parse_optional();
        return Ok(LoopControlFlowStatementAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_exit_statement_final_action(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopControlFlowStatementFinalPartAst> {
        let p1 = self.parse_keyword(Keywords::Skip).enum_wrapper(Box::new(LoopControlFlowStatementFinalPartAst::Skip));
        let p2 = self.parse_expression().enum_wrapper(Box::new(LoopControlFlowStatementFinalPartAst::Expression));
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_skip_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Skip).parse_once()?;
        return Ok(LoopControlFlowStatementAst::new(c1, vec![], Some(LoopControlFlowStatementFinalPartAst::Skip(p1))));
    }

    #[parser_rule]
    fn parse_pin_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<PinStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Pin).parse_once()?;
        let p2 = self.parse_expression().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        return Ok(PinStatementAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_rel_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<RelStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Rel).parse_once()?;
        let p2 = self.parse_expression().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        return Ok(RelStatementAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_inner_scope(self: &'static Rc<Self>) -> SingleParserRuleHandler<InnerScopeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkLeftCurlyBrace).parse_once()?;
        let p2 = self.parse_statement().parse_zero_or_more(Box::new(self.parse_token_no_token()));
        let p3 = self.parse_token_primitive(TokenType::TkRightCurlyBrace).parse_once()?;
        return Ok(InnerScopeAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<StatementAst> {
        let p1 = self.parse_use_statement().enum_wrapper(Box::new(StatementAst::Use));
        let p2 = self.parse_let_statement().enum_wrapper(Box::new(StatementAst::Let));
        let p3 = self.parse_ret_statement().enum_wrapper(Box::new(StatementAst::Ret));
        let p4 = self.parse_exit_statement().enum_wrapper(Box::new(StatementAst::LoopControlFlow));
        let p5 = self.parse_skip_statement().enum_wrapper(Box::new(StatementAst::LoopControlFlow));
        let p6 = self.parse_pin_statement().enum_wrapper(Box::new(StatementAst::Pin));
        let p7 = self.parse_rel_statement().enum_wrapper(Box::new(StatementAst::Rel));
        let p8 = self.parse_assignment_statement().enum_wrapper(Box::new(StatementAst::Assignment));
        let p9 = self.parse_expression().enum_wrapper(Box::new(StatementAst::Expression));
        let p10 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).or(p8).or(p9).parse_once()?;
        return Ok(p10);
    }

    #[parser_rule]
    fn parse_global_use_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<UseStatementAst> {
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let mut p2 = self.parse_use_statement().parse_once()?;
        p2.annotations = p1;
        return Ok(p2);
    }

    #[parser_rule]
    fn parse_use_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<UseStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Use).parse_once()?;
        let p2 = self.parse_upper_identifier().parse_once()?;
        let p3 = self.parse_generic_parameters().parse_optional();
        let p4 = self.parse_token_assign().parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return Ok(UseStatementAst::new(c1, vec![], p1, TypeAst::from(p2), p3, p4, p5));
    }

    #[parser_rule]
    fn parse_global_constant(self: &'static Rc<Self>) -> SingleParserRuleHandler<GlobalConstantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(Box::new(self.parse_token_newline()));
        let p2 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        let p6 = self.parse_token_assign().parse_once()?;
        let p7 = self.parse_global_constant_value().parse_once()?;
        return Ok(GlobalConstantAst::new(c1, p1, p2, p3, p4, p5, p6, p7));
    }

    #[parser_rule]
    fn parse_let_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<LetStatementAst> {
        let p1 = self.parse_let_statement_initialized();
        let p2 = self.parse_let_statement_uninitialized();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_let_statement_initialized(self: &'static Rc<Self>) -> SingleParserRuleHandler<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Let).parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token_assign().parse_once()?;
        let p4 = self.parse_expression().parse_once()?;
        return Ok(LetStatementAst::new_initialized(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_let_statement_uninitialized(self: &'static Rc<Self>) -> SingleParserRuleHandler<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Let).parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return Ok(LetStatementAst::new_uninitialized(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_local_variable(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableAst> {
        let p1 = self.parse_local_variable_destructure_array().enum_wrapper(Box::new(LocalVariableAst::DestructureArray));
        let p2 = self.parse_local_variable_destructure_tuple().enum_wrapper(Box::new(LocalVariableAst::DestructureTuple));
        let p3 = self.parse_local_variable_destructure_object().enum_wrapper(Box::new(LocalVariableAst::DestructureObject));
        let p4 = self.parse_local_variable_single_identifier().enum_wrapper(Box::new(LocalVariableAst::SingleIdentifier));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_local_variable_destructure_skip_argument(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_underscore().parse_once()?;
        return Ok(LocalVariableDestructureSkip1ArgumentAst::new(c1, p1));
    }

    #[parser_rule]
    fn parse_local_variable_destructure_skip_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_local_variable_single_identifier().parse_optional();
        return Ok(LocalVariableDestructureSkipNArgumentsAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_local_variable_single_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_local_variable_single_identifier_alias().parse_optional();
        return Ok(LocalVariableSingleIdentifierAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_local_variable_single_identifier_alias(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableSingleIdentifierAliasAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::As).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(LocalVariableSingleIdentifierAliasAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_local_variable_destructure_array(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self.parse_local_variable_nested_for_destructure_array().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return Ok(LocalVariableDestructureArrayAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_local_variable_destructure_tuple(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_local_variable_nested_for_destructure_tuple().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LocalVariableDestructureTupleAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_local_variable_destructure_object(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token_left_parenthesis().parse_once()?;
        let p3 = self.parse_local_variable_nested_for_destructure_object().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LocalVariableDestructureObjectAst::new(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_local_variable_attribute_binding(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_local_variable_nested_for_attribute_binding().parse_once()?;
        return Ok(LocalVariableAttributeBindingAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_array(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableNestedForDestructureArrayAst> {
        let p1 = self.parse_local_variable_destructure_array().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::DestructureArray));
        let p2 = self.parse_local_variable_destructure_tuple().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::DestructureTuple));
        let p3 = self.parse_local_variable_destructure_object().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::DestructureObject));
        let p4 = self.parse_local_variable_single_identifier().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::SingleIdentifier));
        let p5 = self.parse_local_variable_destructure_skip_arguments().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::SkipNArgs));
        let p6 = self.parse_local_variable_destructure_skip_argument().enum_wrapper(Box::new(LocalVariableNestedForDestructureArrayAst::Skip1Args));
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_tuple(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableNestedForDestructureTupleAst> {
        let p1 = self.parse_local_variable_destructure_array().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::DestructureArray));
        let p2 = self.parse_local_variable_destructure_tuple().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::DestructureTuple));
        let p3 = self.parse_local_variable_destructure_object().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::DestructureObject));
        let p4 = self.parse_local_variable_single_identifier().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::SingleIdentifier));
        let p5 = self.parse_local_variable_destructure_skip_arguments().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::SkipNArgs));
        let p6 = self.parse_local_variable_destructure_skip_argument().enum_wrapper(Box::new(LocalVariableNestedForDestructureTupleAst::Skip1Args));
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_object(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableNestedForDestructureObjectAst> {
        let p1 = self.parse_local_variable_attribute_binding().enum_wrapper(Box::new(LocalVariableNestedForDestructureObjectAst::AttrBind));
        let p2 = self.parse_local_variable_single_identifier().enum_wrapper(Box::new(LocalVariableNestedForDestructureObjectAst::SingleIdentifier));
        let p3 = self.parse_local_variable_destructure_skip_arguments().enum_wrapper(Box::new(LocalVariableNestedForDestructureObjectAst::SkipNArgs));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_local_variable_nested_for_attribute_binding(self: &'static Rc<Self>) -> SingleParserRuleHandler<LocalVariableNestedForAttributeBindingAst> {
        let p1 = self.parse_local_variable_destructure_array().enum_wrapper(Box::new(LocalVariableNestedForAttributeBindingAst::DestructureArray));
        let p2 = self.parse_local_variable_destructure_tuple().enum_wrapper(Box::new(LocalVariableNestedForAttributeBindingAst::DestructureTuple));
        let p3 = self.parse_local_variable_destructure_object().enum_wrapper(Box::new(LocalVariableNestedForAttributeBindingAst::DestructureObject));
        let p4 = self.parse_local_variable_single_identifier().enum_wrapper(Box::new(LocalVariableNestedForAttributeBindingAst::SingleIdentifier));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_assignment_statement(self: &'static Rc<Self>) -> SingleParserRuleHandler<AssignmentStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_expression().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        return Ok(AssignmentStatementAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_case_expression_branch_simple(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_else_case();
        let p2 = self.parse_pattern_statement_flavour_else();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_case_expression_branch(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_destructuring();
        let p2 = self.parse_pattern_statement_flavour_non_destructuring();
        let p3 = self.parse_pattern_statement_flavour_else_case();
        let p4 = self.parse_pattern_statement_flavour_else();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_pattern_statement_flavour_destructuring(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Is).parse_once()?;
        let p2 = self.parse_pattern_group_destructure().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_pattern_guard().parse_optional();
        let p4 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, p3, p4));
    }

    #[parser_rule]
    fn parse_pattern_statement_flavour_non_destructuring(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_boolean_comparison_op().parse_once()?;
        let p2 = self.parse_pattern_variant_expression().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, None, p3));
    }

    #[parser_rule]
    fn parse_pattern_statement_flavour_else_case(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else_case().parse_once()?;
        return Ok(CaseExpressionBranchAst::new_from_else_to_else_case(c1, p1));
    }

    #[parser_rule]
    fn parse_pattern_statement_flavour_else(self: &'static Rc<Self>) -> SingleParserRuleHandler<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else().parse_once()?;
        let p2 = self.parse_inner_scope().parse_once()?;
        return Ok(CaseExpressionBranchAst::new(c1, None, vec![p1], None, p2));
    }

    #[parser_rule]
    fn parse_pattern_group_destructure(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantAst> {
        let p1 = self.parse_pattern_variant_destructure_array().enum_wrapper(Box::new(PatternVariantAst::DestructureArray));
        let p2 = self.parse_pattern_variant_destructure_tuple().enum_wrapper(Box::new(PatternVariantAst::DestructureTuple));
        let p3 = self.parse_pattern_variant_destructure_object().enum_wrapper(Box::new(PatternVariantAst::DestructureObject));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_pattern_variant_skip_argument(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_underscore().parse_once()?;
        return Ok(PatternVariantDestructureSkip1ArgumentAst::new(c1, p1));
    }

    #[parser_rule]
    fn parse_pattern_variant_skip_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_pattern_variant_single_identifier().parse_optional();
        return Ok(PatternVariantDestructureSkipNArgumentsAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_pattern_variant_single_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_local_variable_single_identifier_alias().parse_optional();
        return Ok(PatternVariantSingleIdentifierAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_pattern_variant_destructure_tuple(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_pattern_variant_nested_for_destructure_tuple().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(PatternVariantDestructureTupleAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_pattern_variant_destructure_array(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_pattern_variant_nested_for_destructure_array().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(PatternVariantDestructureArrayAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_pattern_variant_destructure_object(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token_left_parenthesis().parse_once()?;
        let p3 = self.parse_pattern_variant_nested_for_destructure_object().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(PatternVariantDestructureObjectAst::new(c1, p1, p2, p3, p4));
    }

    #[parser_rule]
    fn parse_pattern_variant_attribute_binding(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_pattern_variant_nested_for_attribute_binding().parse_once()?;
        return Ok(PatternVariantAttributeBindingAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_pattern_variant_literal(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantLiteralAst> {
        let p1 = self.parse_literal_float().enum_wrapper(Box::new(PatternVariantLiteralAst::Float));
        let p2 = self.parse_literal_integer().enum_wrapper(Box::new(PatternVariantLiteralAst::Integer));
        let p3 = self.parse_literal_string().enum_wrapper(Box::new(PatternVariantLiteralAst::String));
        let p4 = self.parse_literal_boolean().enum_wrapper(Box::new(PatternVariantLiteralAst::Boolean));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_pattern_variant_expression(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_once()?;
        return Ok(PatternVariantAst::Expression(PatternVariantExpressionAst::new(c1, p1)));
    }

    #[parser_rule]
    fn parse_pattern_variant_else(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Else).parse_once()?;
        return Ok(PatternVariantAst::Else(PatternVariantElseAst::new(c1, p1)));
    }

    #[parser_rule]
    fn parse_pattern_variant_else_case(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Else).parse_once()?;
        let p2 = self.parse_case_expression().parse_once()?;
        return Ok(PatternVariantAst::ElseCase(PatternVariantElseCaseAst::new(c1, p1, p2)));
    }

    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_array(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantNestedForDestructureArrayAst> {
        let p1 = self.parse_pattern_variant_destructure_array().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::DestructureArray));
        let p2 = self.parse_pattern_variant_destructure_tuple().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::DestructureTuple));
        let p3 = self.parse_pattern_variant_destructure_object().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::DestructureObject));
        let p4 = self.parse_pattern_variant_single_identifier().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::SingleIdentifier));
        let p5 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::Literal));
        let p6 = self.parse_pattern_variant_skip_arguments().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::SkipNArgs));
        let p7 = self.parse_pattern_variant_skip_argument().enum_wrapper(Box::new(PatternVariantNestedForDestructureArrayAst::Skip1Args));
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(p8);
    }

    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_tuple(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantNestedForDestructureTupleAst> {
        let p1 = self.parse_pattern_variant_destructure_array().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::DestructureArray));
        let p2 = self.parse_pattern_variant_destructure_tuple().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::DestructureTuple));
        let p3 = self.parse_pattern_variant_destructure_object().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::DestructureObject));
        let p4 = self.parse_pattern_variant_single_identifier().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::SingleIdentifier));
        let p5 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::Literal));
        let p6 = self.parse_pattern_variant_skip_arguments().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::SkipNArgs));
        let p7 = self.parse_pattern_variant_skip_argument().enum_wrapper(Box::new(PatternVariantNestedForDestructureTupleAst::Skip1Args));
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(p8);
    }

    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_object(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantNestedForDestructureObjectAst> {
        let p1 = self.parse_pattern_variant_attribute_binding().enum_wrapper(Box::new(PatternVariantNestedForDestructureObjectAst::AttrBind));
        let p2 = self.parse_pattern_variant_single_identifier().enum_wrapper(Box::new(PatternVariantNestedForDestructureObjectAst::SingleIdentifier));
        let p3 = self.parse_pattern_variant_skip_arguments().enum_wrapper(Box::new(PatternVariantNestedForDestructureObjectAst::SkipNArgs));
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_pattern_variant_nested_for_attribute_binding(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternVariantNestedForAttributeBindingAst> {
        let p1 = self.parse_pattern_variant_destructure_array().enum_wrapper(Box::new(PatternVariantNestedForAttributeBindingAst::DestructureArray));
        let p2 = self.parse_pattern_variant_destructure_tuple().enum_wrapper(Box::new(PatternVariantNestedForAttributeBindingAst::DestructureTuple));
        let p3 = self.parse_pattern_variant_destructure_object().enum_wrapper(Box::new(PatternVariantNestedForAttributeBindingAst::DestructureObject));
        let p4 = self.parse_pattern_variant_literal().enum_wrapper(Box::new(PatternVariantNestedForAttributeBindingAst::Literal));
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_pattern_guard(self: &'static Rc<Self>) -> SingleParserRuleHandler<PatternGuardAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::And).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        return Ok(PatternGuardAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_binary_op_precedence_level_1(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_keyword(Keywords::Or).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_binary_op_precedence_level_2(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_keyword(Keywords::And).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_binary_op_precedence_level_3(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_keyword(Keywords::Is).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_binary_op_precedence_level_4(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
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

    #[parser_rule]
    fn parse_binary_op_precedence_level_5(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_add_assign();
        let p2 = self.parse_token_sub_assign();
        let p3 = self.parse_token_add();
        let p4 = self.parse_token_sub();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_binary_op_precedence_level_6(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1  = self.parse_token_mul_assign();
        let p2  = self.parse_token_div_assign();
        let p3  = self.parse_token_rem_assign();
        let p4  = self.parse_token_mod_assign();
        let p5  = self.parse_token_exp_assign();
        let p6  = self.parse_token_mul();
        let p7  = self.parse_token_div();
        let p8  = self.parse_token_rem();
        let p9  = self.parse_token_mod();
        let p10 = self.parse_token_exp();
        let p11 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).or(p8).or(p9).or(p10).parse_once()?;
        return Ok(p11);
    }

    #[parser_rule]
    fn parse_boolean_comparison_op(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_eq();
        let p2 = self.parse_token_ne();
        let p3 = self.parse_token_le();
        let p4 = self.parse_token_ge();
        let p5 = self.parse_token_lt();
        let p6 = self.parse_token_gt();
        let p8 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p8);
    }

    #[parser_rule]
    fn parse_unary_op(self: &'static Rc<Self>) -> SingleParserRuleHandler<UnaryExpressionOperatorAst> {
        let p1 = self.parse_unary_op_async_call().parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_unary_op_async_call(self: &'static Rc<Self>) -> SingleParserRuleHandler<UnaryExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Async).parse_once()?;
        return Ok(UnaryExpressionOperatorAst::Async(UnaryExpressionOperatorAsyncAst::new(c1, p1)));
    }

    #[parser_rule]
    fn parse_postfix_op(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let p1 = self.parse_postfix_op_function_call();
        let p2 = self.parse_postfix_op_member_access();
        let p3 = self.parse_postfix_op_early_return();
        let p4 = self.parse_postfix_op_not_keyword();
        let p5 = self.parse_postfix_op_step_keyword();
        let p6 = p1.or(p2).or(p3).or(p4).or(p5).parse_once()?;
        return Ok(p6);
    }

    #[parser_rule]
    fn parse_postfix_op_function_call(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_generic_arguments().parse_optional();
        let p2 = self.parse_function_call_arguments().parse_once()?;
        let p3 = self.parse_token_double_dot().parse_optional();
        return Ok(PostfixExpressionOperatorAst::FunctionCall(PostfixExpressionOperatorFunctionCallAst::new(c1, p1, p2, p3)));
    }

    #[parser_rule]
    fn parse_postfix_op_member_access(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let p1 = self.parse_postfix_op_member_access_runtime();
        let p2 = self.parse_postfix_op_member_access_static();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_postfix_op_member_access_runtime(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_dot().parse_once()?;
        let p2 = self.parse_identifier();
        let p3 = self.parse_numeric_identifier();
        let p4 = p2.or(p3).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::MemberAccess(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p4)));
    }

    #[parser_rule]
    fn parse_postfix_op_member_access_static(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_colon().parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(PostfixExpressionOperatorAst::MemberAccess(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2)));
    }

    #[parser_rule]
    fn parse_postfix_op_early_return(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_question_mark().parse_once()?;
        return Ok(PostfixExpressionOperatorAst::EarlyReturn(PostfixExpressionOperatorEarlyReturnAst::new(c1, p1)));
    }

    #[parser_rule]
    fn parse_postfix_op_not_keyword(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_dot().parse_once()?;
        let p2 = self.parse_keyword(Keywords::Not).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::NotKeyword(PostfixExpressionOperatorNotKeywordAst::new(c1, p1, p2)));
    }

    #[parser_rule]
    fn parse_postfix_op_step_keyword(self: &'static Rc<Self>) -> SingleParserRuleHandler<PostfixExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_keyword(Keywords::Step).parse_once()?;
        return Ok(PostfixExpressionOperatorAst::StepKeyword(PostfixExpressionOperatorStepKeywordAst::new(c1, p1, p2)));
    }

    #[parser_rule]
    fn parse_convention(self: &'static Rc<Self>) -> SingleParserRuleHandler<ConventionAst> {
        let p1 = self.parse_convention_mut();
        let p2 = self.parse_convention_ref();
        let p3 = self.parse_convention_mov();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_convention_mov(self: &'static Rc<Self>) -> SingleParserRuleHandler<ConventionAst> {
        let c1 = self.current_pos();
        return Ok(ConventionAst::new_mov(c1));
    }

    #[parser_rule]
    fn parse_convention_mut(self: &'static Rc<Self>) -> SingleParserRuleHandler<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_borrow().parse_once()?;
        let p2 = self.parse_keyword(Keywords::Mut).parse_once()?;
        return Ok(ConventionAst::new_mut(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_convention_ref(self: &'static Rc<Self>) -> SingleParserRuleHandler<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_borrow().parse_once()?;
        return Ok(ConventionAst::new_ref(c1, p1));
    }

    #[parser_rule]
    fn parse_object_initializer(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_object_initializer_arguments().parse_once()?;
        return Ok(ObjectInitializerAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_object_initializer_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_object_initializer_argument().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_object_initializer_argument(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentAst> {
        let p1 = self.parse_object_initializer_argument_named();
        let p2 = self.parse_object_initializer_argument_unnamed();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_object_initializer_argument_unnamed(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_unnamed(c1, p1, p2));
    }
    #[parser_rule]
    fn parse_object_initializer_argument_named(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_named(c1, p1, p2, Box::new(p3)));
    }

    #[parser_rule]
    fn parse_type(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let p1 = self.parse_type_optional();
        let p2 = self.parse_type_variant();
        let p3 = self.parse_type_tuple();
        let p4 = self.parse_type_single();
        let p5 = p1.or(p2).or(p3).or(p4).parse_once()?;
        return Ok(p5);
    }

    #[parser_rule]
    fn parse_type_optional(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_question_mark().parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        return Ok(TypeOptionalAst::new(c1, p1, p2).to_type());
    }

    #[parser_rule]
    fn parse_type_single(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let p1 = self.parse_type_single_with_namespace();
        let p2 = self.parse_type_single_with_self();
        let p3 = self.parse_type_single_without_namespace_or_self();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_type_single_with_namespace(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        let p2 = self.parse_token_double_colon().parse_once()?;
        let p3 = self.parse_generic_identifier().parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        return Ok(TypeAst::new(c1, p1, p3));
    }

    #[parser_rule]
    fn parse_type_single_with_self(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_self_type_keyword().parse_once()?;
        let p2 = self.parse_types_after_self().parse_optional().unwrap_or_default();
        return Ok(TypeAst::new(c1, vec![], vec![p1].into_iter().chain(p2.into_iter()).collect()));
    }

    #[parser_rule]
    fn parse_types_after_self(self: &'static Rc<Self>) -> SingleParserRuleHandler<Vec<GenericIdentifierAst>> {
        let p1 = self.parse_token_double_colon().parse_once()?;
        let p2 = self.parse_generic_identifier().parse_zero_or_more(Box::new(self.parse_token_double_colon()));
        return Ok(p2);
    }

    #[parser_rule]
    fn parse_type_single_without_namespace_or_self(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_generic_identifier().parse_one_or_more(Box::new(self.parse_token_double_colon()))?;
        return Ok(TypeAst::new(c1, vec![], p1));
    }

    #[parser_rule]
    fn parse_self_type_keyword(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::SelfType).parse_once()?;
        return Ok(GenericIdentifierAst::new(c1, "Self".to_string(), None));
    }

    #[parser_rule]
    fn parse_type_tuple(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_type().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(TypeTupleAst::new(c1, p1, p2, p3).to_type());
    }

    #[parser_rule]
    fn parse_type_non_union(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let p1 = self.parse_type_single();
        let p2 = self.parse_type_tuple();
        let p3 = self.parse_type_optional();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_type_variant(self: &'static Rc<Self>) -> SingleParserRuleHandler<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_non_union().parse_two_or_more(Box::new(self.parse_token_union()))?;
        return Ok(TypeVariantAst::new(c1, p1).to_type());
    }

    #[parser_rule]
    fn parse_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_identifier().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    #[parser_rule]
    fn parse_numeric_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_dec_integer().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    #[parser_rule]
    fn parse_upper_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_upper_identifier().parse_once()?;
        return Ok(IdentifierAst::new(c1, p1.metadata));
    }

    #[parser_rule]
    fn parse_generic_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_arguments().parse_optional();
        return Ok(GenericIdentifierAst::new(c1, p1.value, p2));
    }

    #[parser_rule]
    fn parse_literal(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let p1 = self.parse_literal_float();
        let p2 = self.parse_literal_integer();
        let p3 = self.parse_literal_string();
        let p4 = self.parse_literal_tuple(Rc::new(RefCell::new(|| self.parse_expression())));
        let p5 = self.parse_literal_array(Rc::new(RefCell::new(|| self.parse_expression())));
        let p6 = self.parse_literal_boolean();
        let p7 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).parse_once()?;
        return Ok(p7);
    }

    #[parser_rule]
    fn parse_literal_float(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let p1 = self.parse_literal_float_b10().parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_literal_integer(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let p1 = self.parse_literal_integer_b10();
        let p2 = self.parse_literal_integer_b02();
        let p3 = self.parse_literal_integer_b16();
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_literal_string(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme_string().parse_once()?;
        return Ok(LiteralAst::new_string(c1, p1));
    }

    #[parser_rule]
    fn parse_literal_tuple(self: &'static Rc<Self>, item: Rc<RefCell<dyn FnMut() -> SingleParserRuleHandler<ExpressionAst>>>) -> SingleParserRuleHandler<LiteralAst> {
        let p1 = self.parse_literal_tuple_0_items();
        let p2 = self.parse_literal_tuple_1_items(item.clone());
        let p3 = self.parse_literal_tuple_n_items(item.clone());
        let p4 = p1.or(p2).or(p3).parse_once()?;
        return Ok(p4);
    }

    #[parser_rule]
    fn parse_literal_array(self: &'static Rc<Self>, item: Rc<RefCell<dyn FnMut() -> SingleParserRuleHandler<ExpressionAst>>>) -> SingleParserRuleHandler<LiteralAst> {
        let p1 = self.parse_literal_array_0_items();
        let p2 = self.parse_literal_array_n_items(item.clone());
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_literal_boolean(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::True);
        let p2 = self.parse_keyword(Keywords::False);
        let p3 = p1.or(p2).parse_once()?;
        return Ok(LiteralAst::new_boolean(c1, p3));
    }

    #[parser_rule]
    fn parse_literal_float_b10(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_dec_integer().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p4 = self.parse_lexeme_dec_integer().parse_once()?;
        let p5 = self.parse_float_postfix_type().parse_optional();
        return Ok(LiteralAst::new_float(c1, p1, p2, p3, p4, p5.and_then(|x| Some(TypeAst::from(x)))));
    }

    #[parser_rule]
    fn parse_literal_integer_b10(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_dec_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))));
    }

    #[parser_rule]
    fn parse_literal_integer_b02(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_bin_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))));
    }

    #[parser_rule]
    fn parse_literal_integer_b16(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme_hex_integer().parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))));
    }

    #[parser_rule]
    fn parse_numeric_prefix_op(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_sub();
        let p2 = self.parse_token_add();
        let p3 = p1.or(p2).parse_once()?;
        return Ok(p3);
    }

    #[parser_rule]
    fn parse_integer_postfix_type(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let p1 = self.parse_token_primitive(TokenType::TkUnderscore).parse_once()?;
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
        let p14 = p2.or(p3).or(p4).or(p5).or(p6).or(p7).or(p8).or(p9).or(p10).or(p11).or(p12).or(p13).parse_once()?;
        return Ok(IdentifierAst::new(p14.pos, p14.metadata));
    }

    #[parser_rule]
    fn parse_float_postfix_type(self: &'static Rc<Self>) -> SingleParserRuleHandler<IdentifierAst> {
        let p1 = self.parse_token_primitive(TokenType::TkUnderscore).parse_once()?;
        let p2 = self.parse_characters("f8");
        let p3 = self.parse_characters("f16");
        let p4 = self.parse_characters("f32");
        let p5 = self.parse_characters("f64");
        let p6 = self.parse_characters("f128");
        let p7 = self.parse_characters("f256");
        let p8 = p2.or(p3).or(p4).or(p5).or(p6).or(p7).parse_once()?;
        return Ok(IdentifierAst::new(p8.pos, p8.metadata));
    }

    #[parser_rule]
    fn parse_literal_tuple_0_items(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, vec![], p2));
    }

    #[parser_rule]
    fn parse_literal_tuple_1_items(self: &'static Rc<Self>, item: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<ExpressionAst>>>>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = item.borrow_mut()().parse_once()?;
        let p3 = self.parse_token_comma().parse_once()?;
        let p4 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, vec![p2], p4));
    }

    #[parser_rule]
    fn parse_literal_tuple_n_items(self: &'static Rc<Self>, item: Rc<RefCell<dyn FnMut<(), Output=SingleParserRuleHandler<ExpressionAst>>>>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = item.borrow_mut()().parse_two_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(LiteralAst::new_tuple(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_literal_array_0_items(self: &'static Rc<Self>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkComma).parse_once()?;
        let p4 = self.parse_lexeme_dec_integer().parse_once()?;
        let p5 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(LiteralAst::new_array_0(c1, p1, p2, p3, p4, p5));
    }

    #[parser_rule]
    fn parse_literal_array_n_items(self: &'static Rc<Self>, item: Rc<RefCell<dyn FnMut() -> SingleParserRuleHandler<ExpressionAst>>>) -> SingleParserRuleHandler<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = item.borrow_mut()().parse_one_or_more(Box::new(self.parse_token_comma()))?;
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return Ok(LiteralAst::new_array_n(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_global_constant_value(self: &'static Rc<Self>) -> SingleParserRuleHandler<ExpressionAst> {
        let p1 = self.parse_literal_float().enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p2 = self.parse_literal_integer().enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p3 = self.parse_literal_string().enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p4 = self.parse_literal_tuple(Rc::new(RefCell::new(|| self.parse_global_constant_value()))).enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p5 = self.parse_literal_array(Rc::new(RefCell::new(|| self.parse_global_constant_value()))).enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p6 = self.parse_literal_boolean().enum_wrapper(Box::new(PrimaryExpressionAst::Literal)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p7 = self.parse_global_object_initializer().enum_wrapper(Box::new(PrimaryExpressionAst::ObjectInitializer)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p8 = self.parse_identifier().enum_wrapper(Box::new(PrimaryExpressionAst::Identifier)).enum_wrapper(Box::new(ExpressionAst::Primary));
        let p9 = p1.or(p2).or(p3).or(p4).or(p5).or(p6).or(p7).or(p8).parse_once()?;
        return Ok(p9);
    }

    #[parser_rule]
    fn parse_global_object_initializer(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_global_object_initializer_arguments().parse_once()?;
        return Ok(ObjectInitializerAst::new(c1, p1, p2));
    }

    #[parser_rule]
    fn parse_global_object_initializer_arguments(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_global_object_initializer_argument_named().parse_zero_or_more(Box::new(self.parse_token_comma()));
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3));
    }

    #[parser_rule]
    fn parse_global_object_initializer_argument_named(self: &'static Rc<Self>) -> SingleParserRuleHandler<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_global_constant_value().parse_once()?;
        return Ok(ObjectInitializerArgumentAst::new_named(c1, p1, p2, Box::new(p3)));
    }

    #[parser_rule]
    fn parse_keyword(self: &'static Rc<Self>, keyword: Keywords) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        for c in keyword.to_string().chars() {
            let p1 = self.parse_character(c);
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, keyword.to_string()));
    }

    #[parser_rule]
    fn parse_token_left_curly_brace(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLeftCurlyBrace).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_right_curly_brace(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkRightCurlyBrace).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_left_parenthesis(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLeftParenthesis).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_right_parenthesis(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkRightParenthesis).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_left_square_bracket(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLeftSquareBracket).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_right_square_bracket(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkRightSquareBracket).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_dot(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_double_colon(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_comma(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkComma).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_colon(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkColon).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_newline(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkNewLine).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_rightward_arrow(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkMinusSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkGreaterThanSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_double_dot(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkDot).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_question_mark(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkQuestionMark).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_borrow(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkAmpersand).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_union(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkVerticalBar).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_eq(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_ne(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkExclamationMark).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_le(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLessThanSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_ge(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkGreaterThanSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_lt(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLessThanSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_gt(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkGreaterThanSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_ss(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkLessThanSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkGreaterThanSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_add_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPlusSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_sub_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkMinusSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_add(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPlusSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_sub(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkMinusSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_mul_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_div_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkForwardSlash).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_rem_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_mod_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_exp_assign(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        let p3 = self.parse_token_primitive(TokenType::TkEqualsSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_mul(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_div(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkForwardSlash).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_rem(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_mod(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkPercentSign).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_exp(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        let p2 = self.parse_token_primitive(TokenType::TkAsterisk).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_underscore(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkUnderscore).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_speech_mark(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkSpeechMark).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_no_token(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        return Ok(TokenAst::new(c1, TokenType::NoToken, "".to_string()));
    }

    #[parser_rule]
    fn parse_lexeme_dec_integer(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkNumber(num)).parse_once()?;
            integer.push(char::from(p1.metadata.as_bytes()[0]));
        }
        if integer.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected decimal integer".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, integer));
    }

    #[parser_rule]
    fn parse_lexeme_bin_integer(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        self.parse_token_primitive(TokenType::TkNumber('0')).parse_once()?;
        self.parse_token_primitive(TokenType::TkCharacter('b')).parse_once()?;
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkNumber(num)).parse_once()?;
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

    #[parser_rule]
    fn parse_lexeme_hex_integer(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        self.parse_token_primitive(TokenType::TkNumber('0')).parse_once()?;
        self.parse_token_primitive(TokenType::TkCharacter('x')).parse_once()?;
        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkNumber(num)).parse_once()?;
            integer.push(char::from(p1.metadata.as_bytes()[0]));
            if !num.is_digit(16) {
                return Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()));
            }
        }
        if integer.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, integer));
    }

    #[parser_rule]
    fn parse_lexeme_string(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        self.parse_token_speech_mark().parse_once()?;
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkCharacter(string)).parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        self.parse_token_speech_mark().parse_once()?;
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    #[parser_rule]
    fn parse_lexeme_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkCharacter(string)).parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected identifier".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    #[parser_rule]
    fn parse_lexeme_upper_identifier(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = self.parse_token_primitive(TokenType::TkCharacter(string)).parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier.len() == 0 {
            return Err(SyntaxError::new(c1, "Expected upper identifier".to_string()));
        }
        if (identifier.as_bytes()[0] as char).is_lowercase() {
            return Err(SyntaxError::new(c1, "Expected upper identifier".to_string()));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, identifier));
    }

    #[parser_rule]
    fn parse_characters(self: &'static Rc<Self>, characters: &str) -> SingleParserRuleHandler<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        for c in characters.chars() {
            let p1 = self.parse_character(c).parse_once()?;
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }
        if identifier != characters {
            return Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", characters, identifier)));
        }
        return Ok(TokenAst::new(c1, TokenType::NoToken, "".to_string()));
    }

    #[parser_rule]
    fn parse_character(self: &'static Rc<Self>, character: char) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::TkCharacter(character)).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_eof(self: &'static Rc<Self>) -> SingleParserRuleHandler<TokenAst> {
        let p1 = self.parse_token_primitive(TokenType::EndOfFile).parse_once()?;
        return Ok(p1);
    }

    #[parser_rule]
    fn parse_token_primitive(mut self: &'static Rc<Self>, token_type: TokenType) -> SingleParserRuleHandler<TokenAst> {
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

        let r = TokenAst::new(self.current_pos(), self.tokens[self.index].clone(), "".to_string());
        self.index += 1;
        return Ok(r);
    }
}
