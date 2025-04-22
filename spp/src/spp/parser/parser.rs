use crate::spp::asts::convention_mut_ast::ConventionMutAst;
use crate::spp::asts::convention_ref_ast::ConventionRefAst;
use crate::spp::asts::use_statement_redux_ast::UseStatementReduxAst;
use crate::spp::asts::annotation_ast::AnnotationAst;
use crate::spp::asts::assignment_statement_ast::AssignmentStatementAst;
use crate::spp::asts::ast::ToBinaryExpression;
use crate::spp::asts::case_expression_ast::CaseExpressionAst;
use crate::spp::asts::case_expression_branch_ast::CaseExpressionBranchAst;
use crate::spp::asts::class_attribute_ast::ClassAttributeAst;
use crate::spp::asts::class_implementation_ast::ClassImplementationAst;
use crate::spp::asts::class_member_ast::ClassMemberAst;
use crate::spp::asts::class_prototype_ast::ClassPrototypeAst;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::function_call_argument::FunctionCallArgumentAst;
use crate::spp::asts::function_call_argument_group_ast::FunctionCallArgumentGroupAst;
use crate::spp::asts::function_implementation_ast::FunctionImplementationAst;
use crate::spp::asts::function_member_ast::FunctionMemberAst;
use crate::spp::asts::function_parameter_ast::FunctionParameterAst;
use crate::spp::asts::function_parameter_group_ast::FunctionParameterGroupAst;
use crate::spp::asts::function_prototype_ast::{FunctionPrototypeAst, FunctionPrototypeBaseAst};
use crate::spp::asts::gen_expression_ast::GenExpressionAst;
use crate::spp::asts::generic_argument_ast::GenericArgumentAst;
use crate::spp::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::spp::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::spp::asts::generic_parameter_ast::GenericParameterAst;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::spp::asts::cmp_statement_ast::CmpStatementAst;
use crate::spp::asts::function_call_argument_named_ast::FunctionCallArgumentNamedAst;
use crate::spp::asts::function_call_argument_unnamed_ast::FunctionCallArgumentUnnamedAst;
use crate::spp::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::spp::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::spp::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::spp::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;
use crate::spp::asts::generic_argument_comp_named_ast::GenericArgumentCompNamedAst;
use crate::spp::asts::generic_argument_comp_unnamed_ast::GenericArgumentCompUnnamedAst;
use crate::spp::asts::generic_argument_type_named_ast::GenericArgumentTypeNamedAst;
use crate::spp::asts::generic_argument_type_unnamed_ast::GenericArgumentTypeUnnamedAst;
use crate::spp::asts::generic_parameter_comp_optional_ast::GenericParameterCompOptionalAst;
use crate::spp::asts::generic_parameter_comp_required_ast::GenericParameterCompRequiredAst;
use crate::spp::asts::generic_parameter_comp_variadic_ast::GenericParameterCompVariadicAst;
use crate::spp::asts::generic_parameter_type_optional_ast::GenericParameterTypeOptionalAst;
use crate::spp::asts::generic_parameter_type_required_ast::GenericParameterTypeRequiredAst;
use crate::spp::asts::generic_parameter_type_variadic_ast::GenericParameterTypeVariadicAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::let_statement_ast::LetStatementAst;
use crate::spp::asts::let_statement_initialized_ast::LetStatementInitializedAst;
use crate::spp::asts::let_statement_uninitialized_ast::LetStatementUninitializedAst;
use crate::spp::asts::literal_array_ast::LiteralArrayAst;
use crate::spp::asts::literal_array_empty_ast::LiteralArrayEmptyAst;
use crate::spp::asts::literal_ast::LiteralAst;
use crate::spp::asts::literal_boolean_ast::LiteralBooleanAst;
use crate::spp::asts::literal_float_ast::LiteralFloatAst;
use crate::spp::asts::literal_integer_ast::LiteralIntegerAst;
use crate::spp::asts::literal_string_ast::LiteralStringAst;
use crate::spp::asts::literal_tuple_ast::LiteralTupleAst;
use crate::spp::asts::local_variable_ast::{
    LocalVariableAst, LocalVariableNestedForAttributeBindingAst,
    LocalVariableNestedForDestructureArrayAst, LocalVariableNestedForDestructureObjectAst,
    LocalVariableNestedForDestructureTupleAst,
};
use crate::spp::asts::local_variable_attribute_binding_ast::LocalVariableAttributeBindingAst;
use crate::spp::asts::local_variable_destructure_array_ast::LocalVariableDestructureArrayAst;
use crate::spp::asts::local_variable_destructure_object_ast::LocalVariableDestructureObjectAst;
use crate::spp::asts::local_variable_destructure_skip_1_argument_ast::LocalVariableDestructureSkip1ArgumentAst;
use crate::spp::asts::local_variable_destructure_skip_n_arguments_ast::LocalVariableDestructureSkipNArgumentsAst;
use crate::spp::asts::local_variable_destructure_tuple_ast::LocalVariableDestructureTupleAst;
use crate::spp::asts::local_variable_single_identifier_alias_ast::LocalVariableSingleIdentifierAliasAst;
use crate::spp::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use crate::spp::asts::loop_condition_ast::LoopConditionAst;
use crate::spp::asts::loop_condition_boolean_ast::LoopConditionBooleanAst;
use crate::spp::asts::loop_condition_iterable_ast::LoopConditionIterableAst;
use crate::spp::asts::loop_control_flow_statement_ast::LoopControlFlowStatementAst;
use crate::spp::asts::loop_control_flow_statement_final_part_ast::LoopControlFlowStatementFinalPartAst;
use crate::spp::asts::loop_else_statement_ast::LoopElseStatementAst;
use crate::spp::asts::loop_expression_ast::LoopExpressionAst;
use crate::spp::asts::module_implementation_ast::ModuleImplementationAst;
use crate::spp::asts::module_member_ast::ModuleMemberAst;
use crate::spp::asts::module_prototype_ast::ModulePrototypeAst;
use crate::spp::asts::object_initializer::ObjectInitializerAst;
use crate::spp::asts::object_initializer_argument_ast::ObjectInitializerArgumentAst;
use crate::spp::asts::object_initializer_argument_group_ast::ObjectInitializerArgumentGroupAst;
use crate::spp::asts::object_initializer_argument_named::ObjectInitializerArgumentNamedAst;
use crate::spp::asts::object_initializer_argument_unnamed::ObjectInitializerArgumentUnnamedAst;
use crate::spp::asts::parenthesized_expression_ast::ParenthesizedExpressionAst;
use crate::spp::asts::pattern_guard_ast::PatternGuardAst;
use crate::spp::asts::pattern_variant_ast::{
    PatternVariantAst, PatternVariantNestedForAttributeBindingAst,
    PatternVariantNestedForDestructureArrayAst, PatternVariantNestedForDestructureObjectAst,
    PatternVariantNestedForDestructureTupleAst,
};
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
use crate::spp::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::spp::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;
use crate::spp::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::spp::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::spp::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::spp::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;
use crate::spp::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::spp::asts::ret_statement_ast::RetStatementAst;
use crate::spp::asts::statement_ast::StatementAst;
use crate::spp::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::spp::asts::sup_implementation_ast::SupImplementationAst;
use crate::spp::asts::sup_member_ast::SupMemberAst;
use crate::spp::asts::sup_method_prototype_ast::SupMethodPrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::sup_cmp_statement_ast::SupCmpStatementAst;
use crate::spp::asts::sup_use_statement_ast::SupUseStatementAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_array_ast::TypeArrayAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_binary_expression_ast::TypeBinaryExpressionAst;
use crate::spp::asts::type_parenthesized_expression_ast::TypeParenthesizedExpressionAst;
use crate::spp::asts::type_postfix_expression::TypePostfixExpressionAst;
use crate::spp::asts::type_postfix_expression_operator_ast::TypePostfixExpressionOperatorAst;
use crate::spp::asts::type_postfix_expression_operator_nested_ast::TypePostfixExpressionOperatorNestedAst;
use crate::spp::asts::type_postfix_expression_operator_optional_ast::TypePostfixExpressionOperatorOptionalAst;
use crate::spp::asts::type_single_ast::TypeSingleAst;
use crate::spp::asts::type_tuple_ast::TypeTupleAst;
use crate::spp::asts::type_unary_expression_ast::TypeUnaryExpressionAst;
use crate::spp::asts::type_unary_expression_operator_ast::TypeUnaryExpressionOperatorAst;
use crate::spp::asts::type_unary_expression_operator_namespace_ast::TypeUnaryExpressionOperatorNamespaceAst;
use crate::spp::asts::unary_expression_ast::UnaryExpressionAst;
use crate::spp::asts::unary_expression_operator_ast::UnaryExpressionOperatorAst;
use crate::spp::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;
use crate::spp::asts::use_statement_alias_ast::UseStatementAliasAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;
use crate::spp::asts::where_constraints_ast::WhereConstraintsAst;
use crate::spp::asts::where_constraints_group_ast::WhereConstraintsGroupAst;
use crate::spp::lexer::token::{Keywords, TokenAstTokenType, TokenStream, TokenType};
use crate::spp::parser::parser_error::SyntaxError;

pub type ParserResult<T> = Result<T, SyntaxError>;
type ParserRule<T> = fn(&mut Parser) -> ParserResult<T>;


#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: TokenStream,
    pub index: usize,
    token_len: usize,
    error: SyntaxError,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Self {
        let token_len = token_stream.len();
        Self {
            tokens: token_stream,
            index: 0,
            token_len,
            error: SyntaxError::new(0, "".to_string()),
        }
    }

    fn current_pos(&self) -> usize {
        self.index
    }

    fn store_error(&mut self, pos: usize, message: String) -> bool {
        if pos > self.error.get_pos() {
            self.error.reset(pos, message);
            true
        } else {
            false
        }
    }
}

macro_rules! parse_once {
    ($self:ident, $method:expr) => {
        $method($self)?
    };
}

macro_rules! parse_optional {
    ($self:ident, $method:expr) => {{
        let index = $self.index;
        let result = $method($self);

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
        let mut done_1_parse = false;
        let mut result = vec![];
        let mut temp_index = $self.index;
        loop {
            if done_1_parse {
                let sep = parse_optional!($self, $sep);
                if sep.is_none() { break result; }
            }

            let ast = $method($self);
            match ast {
                Ok(ast) => {
                    result.push(ast);
                    done_1_parse = true;
                    temp_index = $self.index;
                }
                Err(_) => {
                    $self.index = temp_index;
                    break result;
                }
            }
        }
    }};
}

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

macro_rules! parse_two_or_more {
    ($self:ident, $method:expr, $sep:expr) => {{
        let result = parse_zero_or_more!($self, $method, $sep);
        if result.len() < 2 {
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
        $method($self)?
    };
    ($self:ident, $method:expr, $($rest:expr),+) => {
        match parse_optional!($self, $method) {
            Some(p) => p,
            None => parse_alternate!($self, $($rest),+),
        }
    };
}

macro_rules! parse_alternate_with_wrapping {
    ($self:ident, $method:expr) => {
        $method.0($method.1($self)?)
    };
    ($self:ident, $method:expr, $($rest:expr),+) => {
        match parse_optional_with_wrapping!($self, $method) {
            Some(p) => p,
            None => parse_alternate_with_wrapping!($self, $($rest),+),
        }
    };
}

macro_rules! parse_optional_with_wrapping {
    ($self:ident, $method:expr) => {{
        let index = $self.index;
        let result = $method.1($self);

        match result {
            Ok(ast) => Some($method.0(ast)),
            Err(_) => {
                $self.index = index;
                None
            }
        }
    }};
}


impl Parser {
    pub fn parse(&mut self) -> ParserResult<ModulePrototypeAst> {
        self.parse_root()
    }

    pub fn parse_root(&mut self) -> ParserResult<ModulePrototypeAst> {
        let p1 = parse_once!(self, Parser::parse_module_prototype);
        let _a = parse_once!(self, Parser::parse_eof);
        Ok(p1)
    }

    pub fn parse_module_prototype(&mut self) -> ParserResult<ModulePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_module_implementation);
        Ok(ModulePrototypeAst::new(c1, p1))
    }

    pub fn parse_module_implementation(&mut self) -> ParserResult<ModuleImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_module_member, Parser::parse_nothing);
        Ok(ModuleImplementationAst::new(c1, p1))
    }

    pub fn parse_module_member(&mut self) -> ParserResult<ModuleMemberAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (ModuleMemberAst::Fun, Parser::parse_function_prototype),
            (ModuleMemberAst::Cmp, Parser::parse_global_cmp_statement),
            (ModuleMemberAst::Use, Parser::parse_global_use_statement),
            (ModuleMemberAst::Cls, Parser::parse_class_prototype),
            (ModuleMemberAst::Ext, Parser::parse_sup_prototype_extension),
            (ModuleMemberAst::Sup, Parser::parse_sup_prototype_functions));
        Ok(p1)
    }

    pub fn parse_class_prototype(&mut self) -> ParserResult<ClassPrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_keyword_cls);
        let p3 = parse_once!(self, Parser::parse_upper_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_optional!(self, Parser::parse_where_block);
        let p6 = parse_once!(self, Parser::parse_class_implementation);
        Ok(ClassPrototypeAst::new(c1, p1, p2, TypeAst::from(p3), p4, p5, p6))
    }

    pub fn parse_class_implementation(&mut self) -> ParserResult<ClassImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_class_member, Parser::parse_nothing);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(ClassImplementationAst::new(c1, p1, p2, p3))
    }

    pub fn parse_class_member(&mut self) -> ParserResult<ClassMemberAst> {
        let p1 = parse_once!(self, Parser::parse_class_attribute);
        Ok(ClassMemberAst::Attr(p1))
    }

    pub fn parse_class_attribute(&mut self) -> ParserResult<ClassAttributeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        let p5 = parse_optional!(self, Parser::parse_class_attribute_default_value);
        Ok(ClassAttributeAst::new(c1, p1, p2, p3, p4, p5))
    }

    pub fn parse_class_attribute_default_value(&mut self) -> ParserResult<ExpressionAst> {
        let _a = parse_once!(self, Parser::parse_token_assign);
        let p2 = parse_once!(self, Parser::parse_expression);
        Ok(p2)
    }

    pub fn parse_sup_prototype_functions(&mut self) -> ParserResult<SupPrototypeFunctionsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_sup);
        let p2 = parse_optional!(self, Parser::parse_generic_parameters);
        let p3 = parse_once!(self, Parser::parse_type);
        let p4 = parse_optional!(self, Parser::parse_where_block);
        let p5 = parse_once!(self, Parser::parse_sup_implementation);
        Ok(SupPrototypeFunctionsAst::new(c1, p1, p2, p3, p4, p5))
    }

    pub fn parse_sup_prototype_extension(&mut self) -> ParserResult<SupPrototypeExtensionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_sup);
        let p2 = parse_optional!(self, Parser::parse_generic_parameters);
        let p3 = parse_once!(self, Parser::parse_type);
        let p4 = parse_once!(self, Parser::parse_keyword_ext);
        let p5 = parse_once!(self, Parser::parse_type);
        let p6 = parse_optional!(self, Parser::parse_where_block);
        let p7 = parse_once!(self, Parser::parse_sup_implementation);
        Ok(SupPrototypeExtensionAst::new(c1, p1, p2, p3, p4, p5, p6, p7))
    }

    pub fn parse_sup_implementation(&mut self) -> ParserResult<SupImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_sup_member, Parser::parse_nothing);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(SupImplementationAst::new(c1, p1, p2, p3))
    }

    pub fn parse_sup_member(&mut self) -> ParserResult<SupMemberAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (SupMemberAst::Fun, Parser::parse_sup_method_prototype),
            (SupMemberAst::Use, Parser::parse_sup_use_statement),
            (SupMemberAst::Cmp, Parser::parse_sup_cmp_statement));
        Ok(p1)
    }

    pub fn parse_sup_method_prototype(&mut self) -> ParserResult<SupMethodPrototypeAst> {
        let p1 = parse_once!(self, Parser::parse_function_prototype);
        Ok(p1)
    }

    pub fn parse_sup_use_statement(&mut self) -> ParserResult<SupUseStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let mut p2 = parse_once!(self, Parser::parse_use_statement);
        match p2 {
            UseStatementAst::Alias(ref mut alias) => { alias.annotations = p1; }
            UseStatementAst::Redux(ref mut redux) => { redux.annotations = p1; }
        }
        Ok(p2)
    }

    pub fn parse_sup_cmp_statement(&mut self) -> ParserResult<SupCmpStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let mut p2 = parse_once!(self, Parser::parse_cmp_statement);
        p2.annotations = p1;
        Ok(p2)
    }

    pub fn parse_function_prototype(&mut self) -> ParserResult<FunctionPrototypeAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (FunctionPrototypeAst::Sub, Parser::parse_subroutine_prototype),
            (FunctionPrototypeAst::Cor, Parser::parse_coroutine_prototype));
        Ok(p1)
    }

    pub fn parse_subroutine_prototype(&mut self) -> ParserResult<SubroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_keyword_fun);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_once!(self, Parser::parse_function_parameters);
        let p6 = parse_once!(self, Parser::parse_token_rightward_arrow);
        let p7 = parse_once!(self, Parser::parse_type);
        let p8 = parse_optional!(self, Parser::parse_where_block);
        let p9 = parse_once!(self, Parser::parse_function_implementation);
        Ok(SubroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9, true)))
    }

    pub fn parse_coroutine_prototype(&mut self) -> ParserResult<CoroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_keyword_cor);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_once!(self, Parser::parse_function_parameters);
        let p6 = parse_once!(self, Parser::parse_token_rightward_arrow);
        let p7 = parse_once!(self, Parser::parse_type);
        let p8 = parse_optional!(self, Parser::parse_where_block);
        let p9 = parse_once!(self, Parser::parse_function_implementation);
        Ok(CoroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9, true)))
    }

    pub fn parse_function_implementation(&mut self) -> ParserResult<FunctionImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_member, Parser::parse_nothing);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(FunctionImplementationAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_member(&mut self) -> ParserResult<FunctionMemberAst> {
        let p1 = parse_once!(self, Parser::parse_statement);
        Ok(p1)
    }

    pub fn parse_function_call_arguments(&mut self) -> ParserResult<FunctionCallArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_call_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(FunctionCallArgumentGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_call_argument(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (FunctionCallArgumentAst::Named, Parser::parse_function_call_argument_named),
            (FunctionCallArgumentAst::Unnamed, Parser::parse_function_call_argument_unnamed));
        Ok(p1)
    }

    pub fn parse_function_call_argument_unnamed(&mut self) -> ParserResult<FunctionCallArgumentUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_convention);
        let p2 = parse_optional!(self, Parser::parse_token_double_dot);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionCallArgumentUnnamedAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_call_argument_named(&mut self) -> ParserResult<FunctionCallArgumentNamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_optional!(self, Parser::parse_convention);
        let p4 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionCallArgumentNamedAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_function_parameters(&mut self) -> ParserResult<FunctionParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_parameter, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(FunctionParameterGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_parameter(&mut self) -> ParserResult<FunctionParameterAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (FunctionParameterAst::Self_, Parser::parse_function_parameter_self_with_arbitrary_type),
            (FunctionParameterAst::Variadic, Parser::parse_function_parameter_variadic),
            (FunctionParameterAst::Optional, Parser::parse_function_parameter_optional),
            (FunctionParameterAst::Required, Parser::parse_function_parameter_required),
            (FunctionParameterAst::Self_, Parser::parse_function_parameter_self));
        Ok(p1)
    }

    pub fn parse_function_parameter_self(&mut self) -> ParserResult<FunctionParameterSelfAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_optional!(self, Parser::parse_convention);
        let p3 = parse_once!(self, Parser::parse_self_identifier);
        Ok(FunctionParameterSelfAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_parameter_self_with_arbitrary_type(&mut self) -> ParserResult<FunctionParameterSelfAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_keyword_self_val);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(FunctionParameterSelfAst::new_with_type(c1, p1, None, IdentifierAst::new(p2.pos, p2.metadata), p3, p4))
    }

    pub fn parse_function_parameter_required(&mut self) -> ParserResult<FunctionParameterRequiredAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_type);
        Ok(FunctionParameterRequiredAst::new(c1, p1, p2, p3))
    }

    pub fn parse_function_parameter_optional(&mut self) -> ParserResult<FunctionParameterOptionalAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_type);
        let p4 = parse_once!(self, Parser::parse_token_assign);
        let p5 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionParameterOptionalAst::new(c1, p1, p2, p3, p4, p5))
    }

    pub fn parse_function_parameter_variadic(&mut self) -> ParserResult<FunctionParameterVariadicAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(FunctionParameterVariadicAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_generic_arguments(&mut self) -> ParserResult<GenericArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_zero_or_more!(self, Parser::parse_generic_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(GenericArgumentGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_generic_argument(&mut self) -> ParserResult<GenericArgumentAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (GenericArgumentAst::TypeNamed, Parser::parse_generic_argument_type_named),
            (GenericArgumentAst::TypeUnnamed, Parser::parse_generic_argument_type_unnamed),
            (GenericArgumentAst::CompNamed, Parser::parse_generic_argument_comp_named),
            (GenericArgumentAst::CompUnnamed, Parser::parse_generic_argument_comp_unnamed));
        Ok(p1)
    }

    pub fn parse_generic_argument_type_named(&mut self) -> ParserResult<GenericArgumentTypeNamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_type);
        Ok(GenericArgumentTypeNamedAst::new(c1, TypeAst::from(p1), p2, p3))
    }

    pub fn parse_generic_argument_type_unnamed(&mut self) -> ParserResult<GenericArgumentTypeUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type);
        Ok(GenericArgumentTypeUnnamedAst::new(c1, p1))
    }

    pub fn parse_generic_argument_comp_named(&mut self) -> ParserResult<GenericArgumentCompNamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_cmp_value);
        Ok(GenericArgumentCompNamedAst::new(c1, TypeAst::from(p1), p2, p3))
    }

    pub fn parse_generic_argument_comp_unnamed(&mut self) -> ParserResult<GenericArgumentCompUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_cmp_value);
        Ok(GenericArgumentCompUnnamedAst::new(c1, p1))
    }

    pub fn parse_generic_parameters(&mut self) -> ParserResult<GenericParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_zero_or_more!(self, Parser::parse_generic_parameter, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(GenericParameterGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_generic_parameter(&mut self) -> ParserResult<GenericParameterAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (GenericParameterAst::TypeVariadic, Parser::parse_generic_parameter_type_variadic),
            (GenericParameterAst::TypeOptional, Parser::parse_generic_parameter_type_optional),
            (GenericParameterAst::TypeRequired, Parser::parse_generic_parameter_type_required),
            (GenericParameterAst::CompVariadic, Parser::parse_generic_parameter_comp_variadic),
            (GenericParameterAst::CompOptional, Parser::parse_generic_parameter_comp_optional),
            (GenericParameterAst::CompRequired, Parser::parse_generic_parameter_comp_required));
        Ok(p1)
    }

    pub fn parse_generic_parameter_type_required(&mut self) -> ParserResult<GenericParameterTypeRequiredAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        Ok(GenericParameterTypeRequiredAst::new(c1, TypeAst::from(p1), p2))
    }

    pub fn parse_generic_parameter_type_optional(&mut self) -> ParserResult<GenericParameterTypeOptionalAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        let p3 = parse_once!(self, Parser::parse_token_assign);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterTypeOptionalAst::new(c1, TypeAst::from(p1), p2, p3, p4))
    }

    pub fn parse_generic_parameter_type_variadic(&mut self) -> ParserResult<GenericParameterTypeVariadicAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_upper_identifier);
        let p3 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        Ok(GenericParameterTypeVariadicAst::new(c1, p1, TypeAst::from(p2), p3))
    }

    pub fn parse_generic_parameter_comp_required(&mut self) -> ParserResult<GenericParameterCompRequiredAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterCompRequiredAst::new(c1, p1, TypeAst::from(p2), p3, p4))
    }

    pub fn parse_generic_parameter_comp_optional(&mut self) -> ParserResult<GenericParameterCompOptionalAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        let p5 = parse_once!(self, Parser::parse_token_assign);
        let p6 = parse_once!(self, Parser::parse_cmp_value);
        Ok(GenericParameterCompOptionalAst::new(c1, p1, TypeAst::from(p2), p3, p4, p5, p6))
    }

    pub fn parse_generic_parameter_comp_variadic(&mut self) -> ParserResult<GenericParameterCompVariadicAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_token_double_dot);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_once!(self, Parser::parse_token_colon);
        let p5 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterCompVariadicAst::new(c1, p1, p2, TypeAst::from(p3), p4, p5))
    }

    pub fn parse_generic_inline_constraints(&mut self) -> ParserResult<GenericParameterConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_colon);
        let p2 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterConstraintsAst::new(c1, p1, p2))
    }

    pub fn parse_where_block(&mut self) -> ParserResult<WhereBlockAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_where);
        let p2 = parse_once!(self, Parser::parse_where_block_constraints_group);
        Ok(WhereBlockAst::new(c1, p1, p2))
    }

    pub fn parse_where_block_constraints_group(&mut self) -> ParserResult<WhereConstraintsGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_where_block_constraints, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(WhereConstraintsGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_where_block_constraints(&mut self) -> ParserResult<WhereConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_type, Parser::parse_token_comma);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_type);
        Ok(WhereConstraintsAst::new(c1, p1, p2, p3))
    }

    pub fn parse_annotation(&mut self) -> ParserResult<AnnotationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_at);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(AnnotationAst::new(c1, p1, p2))
    }

    pub fn parse_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_once!(self, Parser::parse_binary_expression_precedence_level_1);
        Ok(p1)
    }

    pub fn parse_binary_expression_precedence_level_n_rhs<T: ToBinaryExpression + 'static>(&mut self, op: ParserRule<TokenAst>, rhs: ParserRule<T>) -> ParserResult<(TokenAst, T)> {
        let p1 = parse_once!(self, op);
        let p2 = parse_once!(self, rhs);
        Ok((p1, p2))
    }

    pub fn parse_binary_expression_precedence_level_n<T: ToBinaryExpression + 'static>(&mut self, lhs: ParserRule<ExpressionAst>, op: ParserRule<TokenAst>, rhs: ParserRule<T>) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, lhs);
        let p2 = parse_optional!(self, |s| Parser::parse_binary_expression_precedence_level_n_rhs(s, op, rhs));
        Ok(if let Some(p2) = p2 { T::to_binary_expression(c1, p1, p2.0, p2.1) } else { p1 })
    }

    pub fn parse_binary_expression_precedence_level_1(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_2,
            Parser::parse_binary_op_precedence_level_1,
            Parser::parse_binary_expression_precedence_level_1)
    }

    pub fn parse_binary_expression_precedence_level_2(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_3,
            Parser::parse_binary_op_precedence_level_2,
            Parser::parse_binary_expression_precedence_level_2)
    }

    pub fn parse_binary_expression_precedence_level_3(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_4,
            Parser::parse_binary_op_precedence_level_3,
            Parser::parse_pattern_group_destructure)
    }

    pub fn parse_binary_expression_precedence_level_4(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_5,
            Parser::parse_binary_op_precedence_level_4,
            Parser::parse_binary_expression_precedence_level_4)
    }

    pub fn parse_binary_expression_precedence_level_5(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_binary_expression_precedence_level_6,
            Parser::parse_binary_op_precedence_level_5,
            Parser::parse_binary_expression_precedence_level_5)
    }

    pub fn parse_binary_expression_precedence_level_6(&mut self) -> ParserResult<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            Parser::parse_unary_expression,
            Parser::parse_binary_op_precedence_level_6,
            Parser::parse_binary_expression_precedence_level_6)
    }

    pub fn parse_unary_expression(&mut self) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_unary_op, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_postfix_expression);
        p1.into_iter().rev().fold(Ok(p2), |acc, x| { Ok(ExpressionAst::Unary(UnaryExpressionAst::new(c1, x, Box::from(acc?), ))) })
    }

    pub fn parse_postfix_expression(&mut self) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_primary_expression);
        let p2 = parse_zero_or_more!(self, Parser::parse_postfix_op, Parser::parse_nothing);
        p2.into_iter().fold(Ok(p1), |acc, x| { Ok(ExpressionAst::Postfix(PostfixExpressionAst::new(c1, Box::from(acc?), x, ))) })
    }

    pub fn parse_primary_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::ObjectInitializer(x)), Parser::parse_object_initializer),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Parenthesized(x)), Parser::parse_parenthesized_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Case(x)), Parser::parse_case_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Loop(x)), Parser::parse_loop_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Gen(x)), Parser::parse_gen_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Type(x)), Parser::parse_type),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Identifier(x)), Parser::parse_self_identifier),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Identifier(x)), Parser::parse_identifier),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::InnerScope(x)), Parser::parse_inner_scope),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Fold(x)), Parser::parse_fold_expression));
        Ok(p1)
    }

    pub fn parse_parenthesized_expression(&mut self) -> ParserResult<ParenthesizedExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ParenthesizedExpressionAst::new(c1, p1, Box::from(p2), p3))
    }

    pub fn parse_self_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_self_val);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    pub fn parse_fold_expression(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        Ok(p1)
    }

    pub fn parse_case_expression(&mut self) -> ParserResult<CaseExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_case_expression_patterns,
            Parser::parse_case_expression_simple);
        Ok(p1)
    }

    pub fn parse_case_expression_patterns(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_case);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_keyword_of);
        let p4 = parse_one_or_more!(self, Parser::parse_case_expression_branch, Parser::parse_nothing);
        Ok(CaseExpressionAst::new(c1, p1, Box::from(p2), p3, p4))
    }

    pub fn parse_case_expression_simple(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_case);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        let p4 = parse_zero_or_more!(self, Parser::parse_case_expression_branch_simple, Parser::parse_nothing);
        Ok(CaseExpressionAst::new_from_simple(c1, p1, Box::from(p2), p3, p4))
    }

    pub fn parse_loop_expression(&mut self) -> ParserResult<LoopExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_loop);
        let p2 = parse_once!(self, Parser::parse_loop_expression_condition);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        let p4 = parse_optional!(self, Parser::parse_loop_else_statement);
        Ok(LoopExpressionAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_loop_expression_condition(&mut self) -> ParserResult<LoopConditionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LoopConditionAst::Boolean, Parser::parse_loop_expression_condition_boolean),
            (LoopConditionAst::Iterable, Parser::parse_loop_expression_condition_iterable));
        Ok(p1)
    }

    pub fn parse_loop_expression_condition_boolean(&mut self) -> ParserResult<LoopConditionBooleanAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_expression);
        Ok(LoopConditionBooleanAst::new(c1, Box::new(p1)))
    }

    pub fn parse_loop_expression_condition_iterable(&mut self) -> ParserResult<LoopConditionIterableAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_keyword_in);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(LoopConditionIterableAst::new(c1, p1, p2, Box::new(p3)))
    }

    pub fn parse_loop_else_statement(&mut self) -> ParserResult<LoopElseStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        let p2 = parse_once!(self, Parser::parse_inner_scope);
        Ok(LoopElseStatementAst::new(c1, p1, p2))
    }

    pub fn parse_gen_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_gen_expression_unroll,
            Parser::parse_gen_expression_normal);
        Ok(p1)
    }

    pub fn parse_gen_expression_normal(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_gen_expression_normal_with_expression,
            Parser::parse_gen_expression_normal_no_expression);
        Ok(p1)
    }

    pub fn parse_gen_expression_normal_no_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        Ok(GenExpressionAst::new(c1, p1, None, None, None, ))
    }

    pub fn parse_gen_expression_normal_with_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        let p2 = parse_optional!(self, Parser::parse_convention);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(GenExpressionAst::new(c1, p1, None, p2, Some(Box::new(p3))))
    }

    pub fn parse_gen_expression_unroll(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        let p2 = parse_once!(self, Parser::parse_keyword_with);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(GenExpressionAst::new(c1, p1, Some(p2), None, Some(Box::new(p3))))
    }

    pub fn parse_ret_statement(&mut self) -> ParserResult<RetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_ret);
        let p2 = parse_optional!(self, Parser::parse_expression);
        Ok(RetStatementAst::new(c1, p1, p2))
    }

    pub fn parse_exit_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_keyword_exit, Parser::parse_nothing);
        let p2 = parse_optional!(self, Parser::parse_exit_statement_final_action);
        Ok(LoopControlFlowStatementAst::new(c1, p1, p2))
    }

    pub fn parse_exit_statement_final_action(&mut self) -> ParserResult<LoopControlFlowStatementFinalPartAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LoopControlFlowStatementFinalPartAst::Skip, Parser::parse_keyword_skip),
            (LoopControlFlowStatementFinalPartAst::Expression, Parser::parse_expression));
        Ok(p1)
    }

    pub fn parse_skip_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_keyword_skip, Parser::parse_nothing);
        Ok(LoopControlFlowStatementAst::new(c1, p1, None))
    }

    pub fn parse_inner_scope(&mut self) -> ParserResult<InnerScopeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_statement, Parser::parse_nothing);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(InnerScopeAst::new(c1, p1, p2, p3))
    }

    pub fn parse_statement(&mut self) -> ParserResult<StatementAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (StatementAst::Use, Parser::parse_use_statement),
            (StatementAst::Let, Parser::parse_let_statement),
            (StatementAst::Ret, Parser::parse_ret_statement),
            (StatementAst::LoopControlFlow , Parser::parse_exit_statement),
            (StatementAst::LoopControlFlow , Parser::parse_skip_statement),
            (StatementAst::Assignment, Parser::parse_assignment_statement),
            (StatementAst::Expression, Parser::parse_expression));
        Ok(p1)
    }

    pub fn parse_global_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let mut p2 = parse_once!(self, Parser::parse_use_statement);
        match p2 {
            UseStatementAst::Alias(ref mut x) => { x.annotations = p1; }
            UseStatementAst::Redux(ref mut x) => { x.annotations = p1; }
        }
        Ok(p2)
    }

    pub fn parse_use_statement_alias(&mut self) -> ParserResult<UseStatementAliasAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_use);
        let p2 = parse_once!(self, Parser::parse_upper_identifier);
        let p3 = parse_optional!(self, Parser::parse_generic_parameters);
        let p4 = parse_once!(self, Parser::parse_token_assign);
        let p5 = parse_once!(self, Parser::parse_type);
        Ok(UseStatementAliasAst::new(c1, vec![], p1, TypeAst::from(p2), p3, p4, p5))
    }

    pub fn parse_use_statement_redux(&mut self) -> ParserResult<UseStatementReduxAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_use);
        let p2 = parse_once!(self, Parser::parse_type_simple);
        Ok(UseStatementReduxAst::new(c1, vec![], p1, p2))
    }

    pub fn parse_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (UseStatementAst::Alias, Parser::parse_use_statement_alias),
            (UseStatementAst::Redux, Parser::parse_use_statement_redux));
        Ok(p1)
    }

    pub fn parse_global_cmp_statement(&mut self) -> ParserResult<CmpStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let mut p2 = parse_once!(self, Parser::parse_cmp_statement);
        p2.annotations = p1;
        Ok(p2)
    }

    pub fn parse_cmp_statement(&mut self) -> ParserResult<CmpStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_keyword_cmp);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_once!(self, Parser::parse_token_colon);
        let p5 = parse_once!(self, Parser::parse_type);
        let p6 = parse_once!(self, Parser::parse_token_assign);
        let p7 = parse_once!(self, Parser::parse_cmp_value);
        Ok(CmpStatementAst::new(c1, p1, p2, p3, p4, p5, p6, p7))
    }

    pub fn parse_let_statement(&mut self) -> ParserResult<LetStatementAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LetStatementAst::Initialized, Parser::parse_let_statement_initialized),
            (LetStatementAst::Uninitialized, Parser::parse_let_statement_uninitialized));
        Ok(p1)
    }

    pub fn parse_let_statement_initialized(&mut self) -> ParserResult<LetStatementInitializedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_let);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_assign);
        let p4 = parse_once!(self, Parser::parse_expression);
        let p5 = parse_optional!(self, Parser::parse_let_statement_initialized_explicit_type);
        Ok(LetStatementInitializedAst::new(c1, p1, p2, p3, p4, p5))
    }

    pub fn parse_let_statement_initialized_explicit_type(&mut self) -> ParserResult<TypeAst> {
        let _a = parse_once!(self, Parser::parse_token_colon);
        let p2 = parse_once!(self, Parser::parse_type);
        Ok(p2)
    }

    pub fn parse_let_statement_uninitialized(&mut self) -> ParserResult<LetStatementUninitializedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_let);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(LetStatementUninitializedAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_local_variable(&mut self) -> ParserResult<LocalVariableAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableAst::SingleIdentifier, Parser::parse_local_variable_single_identifier));
        Ok(p1)
    }

    pub fn parse_local_variable_destructure_skip_1_argument(&mut self) -> ParserResult<LocalVariableDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_underscore);
        Ok(LocalVariableDestructureSkip1ArgumentAst::new(c1, p1))
    }

    pub fn parse_local_variable_destructure_skip_n_arguments(&mut self) -> ParserResult<LocalVariableDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_optional!(self, Parser::parse_local_variable_single_identifier);
        Ok(LocalVariableDestructureSkipNArgumentsAst::new(c1, p1, p2))
    }

    pub fn parse_local_variable_single_identifier(&mut self) -> ParserResult<LocalVariableSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_optional!(self, Parser::parse_local_variable_single_identifier_alias);
        Ok(LocalVariableSingleIdentifierAst::new(c1, p1, p2, p3))
    }

    pub fn parse_local_variable_single_identifier_alias(&mut self) -> ParserResult<LocalVariableSingleIdentifierAliasAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_as);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(LocalVariableSingleIdentifierAliasAst::new(c1, p1, p2))
    }

    pub fn parse_local_variable_destructure_array(&mut self) -> ParserResult<LocalVariableDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_one_or_more!(self, Parser::parse_local_variable_nested_for_destructure_array, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(LocalVariableDestructureArrayAst::new(c1, p1, p2, p3))
    }

    pub fn parse_local_variable_destructure_tuple(&mut self) -> ParserResult<LocalVariableDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_one_or_more!(self, Parser::parse_local_variable_nested_for_destructure_tuple, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LocalVariableDestructureTupleAst::new(c1, p1, p2, p3))
    }

    pub fn parse_local_variable_destructure_object(&mut self) -> ParserResult<LocalVariableDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_simple);
        let p2 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p3 = parse_zero_or_more!(self, Parser::parse_local_variable_nested_for_destructure_object, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LocalVariableDestructureObjectAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_local_variable_attribute_binding(&mut self) -> ParserResult<LocalVariableAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_local_variable_nested_for_attribute_binding);
        Ok(LocalVariableAttributeBindingAst::new(c1, p1, p2, p3))
    }

    pub fn parse_local_variable_nested_for_destructure_array(&mut self) -> ParserResult<LocalVariableNestedForDestructureArrayAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureArrayAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForDestructureArrayAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForDestructureArrayAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForDestructureArrayAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureArrayAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments),
            (LocalVariableNestedForDestructureArrayAst::Skip1Args, Parser::parse_local_variable_destructure_skip_1_argument));
        Ok(p1)
    }

    pub fn parse_local_variable_nested_for_destructure_tuple(&mut self) -> ParserResult<LocalVariableNestedForDestructureTupleAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureTupleAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForDestructureTupleAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForDestructureTupleAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForDestructureTupleAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureTupleAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments),
            (LocalVariableNestedForDestructureTupleAst::Skip1Args, Parser::parse_local_variable_destructure_skip_1_argument));
        Ok(p1)
    }

    pub fn parse_local_variable_nested_for_destructure_object(&mut self) -> ParserResult<LocalVariableNestedForDestructureObjectAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureObjectAst::AttrBind, Parser::parse_local_variable_attribute_binding),
            (LocalVariableNestedForDestructureObjectAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureObjectAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments));
        Ok(p1)
    }

    pub fn parse_local_variable_nested_for_attribute_binding(&mut self) -> ParserResult<LocalVariableNestedForAttributeBindingAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForAttributeBindingAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForAttributeBindingAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForAttributeBindingAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForAttributeBindingAst::SingleIdentifier, Parser::parse_local_variable_single_identifier));
        Ok(p1)
    }

    pub fn parse_assignment_statement(&mut self) -> ParserResult<AssignmentStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        Ok(AssignmentStatementAst::new(c1, p1, p2, p3))
    }

    pub fn parse_case_expression_branch_simple(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_pattern_statement_flavour_else_case,
            Parser::parse_pattern_statement_flavour_else);
        Ok(p1)
    }

    pub fn parse_case_expression_branch(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_pattern_statement_flavour_destructuring,
            Parser::parse_pattern_statement_flavour_non_destructuring,
            Parser::parse_pattern_statement_flavour_else_case,
            Parser::parse_pattern_statement_flavour_else);
        Ok(p1)
    }

    pub fn parse_pattern_statement_flavour_destructuring(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_is);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_group_destructure, Parser::parse_token_comma);
        let p3 = parse_optional!(self, Parser::parse_pattern_guard);
        let p4 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, p3, p4))
    }

    pub fn parse_pattern_statement_flavour_non_destructuring(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_boolean_comparison_op);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, None, p3))
    }

    pub fn parse_pattern_statement_flavour_else_case(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_pattern_variant_else_case);
        Ok(CaseExpressionBranchAst::new_from_else_to_else_case(c1, p1))
    }

    pub fn parse_pattern_statement_flavour_else(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_pattern_variant_else);
        let p2 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, None, vec![p1], None, p2))
    }

    pub fn parse_pattern_group_destructure(&mut self) -> ParserResult<PatternVariantAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantAst::DestructureObject, Parser::parse_pattern_variant_destructure_object));
        Ok(p1)
    }

    pub fn parse_pattern_variant_skip_argument(&mut self) -> ParserResult<PatternVariantDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_underscore);
        Ok(PatternVariantDestructureSkip1ArgumentAst::new(c1, p1))
    }

    pub fn parse_pattern_variant_skip_arguments(&mut self) -> ParserResult<PatternVariantDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_optional!(self, Parser::parse_pattern_variant_single_identifier);
        Ok(PatternVariantDestructureSkipNArgumentsAst::new(c1, p1, p2))
    }

    pub fn parse_pattern_variant_single_identifier(&mut self) -> ParserResult<PatternVariantSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_optional!(self, Parser::parse_local_variable_single_identifier_alias);
        Ok(PatternVariantSingleIdentifierAst::new(c1, p1, p2, p3))
    }

    pub fn parse_pattern_variant_destructure_array(&mut self) -> ParserResult<PatternVariantDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_array, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(PatternVariantDestructureArrayAst::new(c1, p1, p2, p3))
    }

    pub fn parse_pattern_variant_destructure_tuple(&mut self) -> ParserResult<PatternVariantDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_tuple, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(PatternVariantDestructureTupleAst::new(c1, p1, p2, p3))
    }

    pub fn parse_pattern_variant_destructure_object(&mut self) -> ParserResult<PatternVariantDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_simple);
        let p2 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p3 = parse_zero_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_object, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(PatternVariantDestructureObjectAst::new(c1, p1, p2, p3, p4))
    }

    pub fn parse_pattern_variant_attribute_binding(&mut self) -> ParserResult<PatternVariantAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_pattern_variant_nested_for_attribute_binding);
        Ok(PatternVariantAttributeBindingAst::new(c1, p1, p2, p3))
    }

    pub fn parse_pattern_variant_literal(&mut self) -> ParserResult<PatternVariantLiteralAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantLiteralAst::Float, Parser::parse_literal_float),
            (PatternVariantLiteralAst::Integer, Parser::parse_literal_integer),
            (PatternVariantLiteralAst::String, Parser::parse_literal_string),
            (PatternVariantLiteralAst::Boolean, Parser::parse_literal_boolean));
        Ok(p1)
    }

    pub fn parse_pattern_variant_expression(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_expression);
        Ok(PatternVariantAst::Expression(PatternVariantExpressionAst::new(c1, p1)))
    }

    pub fn parse_pattern_variant_else(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        Ok(PatternVariantAst::Else(PatternVariantElseAst::new(c1, p1)))
    }

    pub fn parse_pattern_variant_else_case(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        let p2 = parse_once!(self, Parser::parse_case_expression);
        Ok(PatternVariantAst::ElseCase(PatternVariantElseCaseAst::new(c1, p1, p2)))
    }

    pub fn parse_pattern_variant_nested_for_destructure_array(&mut self) -> ParserResult<PatternVariantNestedForDestructureArrayAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForDestructureArrayAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantNestedForDestructureArrayAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantNestedForDestructureArrayAst::DestructureObject, Parser::parse_pattern_variant_destructure_object),
            (PatternVariantNestedForDestructureArrayAst::SingleIdentifier, Parser::parse_pattern_variant_single_identifier),
            (PatternVariantNestedForDestructureArrayAst::Literal, Parser::parse_pattern_variant_literal),
            (PatternVariantNestedForDestructureArrayAst::SkipNArgs, Parser::parse_pattern_variant_skip_arguments),
            (PatternVariantNestedForDestructureArrayAst::Skip1Args, Parser::parse_pattern_variant_skip_argument));
        Ok(p1)
    }

    pub fn parse_pattern_variant_nested_for_destructure_tuple(&mut self) -> ParserResult<PatternVariantNestedForDestructureTupleAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForDestructureTupleAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantNestedForDestructureTupleAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantNestedForDestructureTupleAst::DestructureObject, Parser::parse_pattern_variant_destructure_object),
            (PatternVariantNestedForDestructureTupleAst::SingleIdentifier, Parser::parse_pattern_variant_single_identifier),
            (PatternVariantNestedForDestructureTupleAst::Literal, Parser::parse_pattern_variant_literal),
            (PatternVariantNestedForDestructureTupleAst::SkipNArgs, Parser::parse_pattern_variant_skip_arguments),
            (PatternVariantNestedForDestructureTupleAst::Skip1Args, Parser::parse_pattern_variant_skip_argument));
        Ok(p1)
    }

    pub fn parse_pattern_variant_nested_for_destructure_object(&mut self) -> ParserResult<PatternVariantNestedForDestructureObjectAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForDestructureObjectAst::AttrBind, Parser::parse_pattern_variant_attribute_binding),
            (PatternVariantNestedForDestructureObjectAst::SingleIdentifier, Parser::parse_pattern_variant_single_identifier),
            (PatternVariantNestedForDestructureObjectAst::SkipNArgs, Parser::parse_pattern_variant_skip_arguments));
        Ok(p1)
    }

    pub fn parse_pattern_variant_nested_for_attribute_binding(&mut self) -> ParserResult<PatternVariantNestedForAttributeBindingAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForAttributeBindingAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantNestedForAttributeBindingAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantNestedForAttributeBindingAst::DestructureObject, Parser::parse_pattern_variant_destructure_object),
            (PatternVariantNestedForAttributeBindingAst::Literal, Parser::parse_pattern_variant_literal));
        Ok(p1)
    }

    pub fn parse_pattern_guard(&mut self) -> ParserResult<PatternGuardAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_and);
        let p2 = parse_once!(self, Parser::parse_expression);
        Ok(PatternGuardAst::new(c1, p1, p2))
    }

    pub fn parse_binary_op_precedence_level_1(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_or);
        Ok(p1)
    }

    pub fn parse_binary_op_precedence_level_2(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_and);
        Ok(p1)
    }

    pub fn parse_binary_op_precedence_level_3(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_is);
        Ok(p1)
    }

    pub fn parse_binary_op_precedence_level_4(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_eq,
            Parser::parse_token_ne,
            Parser::parse_token_le,
            Parser::parse_token_ge,
            Parser::parse_token_lt,
            Parser::parse_token_gt);
        Ok(p1)
    }

    pub fn parse_binary_op_precedence_level_5(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_add_assign,
            Parser::parse_token_sub_assign,
            Parser::parse_token_add,
            Parser::parse_token_sub);
        Ok(p1)
    }

    pub fn parse_binary_op_precedence_level_6(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_mul_assign,
            Parser::parse_token_div_assign,
            Parser::parse_token_rem_assign,
            Parser::parse_token_mod_assign,
            Parser::parse_token_exp_assign,
            Parser::parse_token_mul,
            Parser::parse_token_div,
            Parser::parse_token_rem,
            Parser::parse_token_mod,
            Parser::parse_token_exp);
        Ok(p1)
    }

    pub fn parse_boolean_comparison_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_eq,
            Parser::parse_token_ne,
            Parser::parse_token_le,
            Parser::parse_token_ge,
            Parser::parse_token_lt,
            Parser::parse_token_gt);
        Ok(p1)
    }

    pub fn parse_unary_op(&mut self) -> ParserResult<UnaryExpressionOperatorAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (UnaryExpressionOperatorAst::Async, Parser::parse_unary_op_async_call));
        Ok(p1)
    }

    pub fn parse_unary_op_async_call(&mut self) -> ParserResult<UnaryExpressionOperatorAsyncAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_async);
        Ok(UnaryExpressionOperatorAsyncAst::new(c1, p1))
    }

    pub fn parse_postfix_op(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PostfixExpressionOperatorAst::FunctionCall, Parser::parse_postfix_op_function_call),
            (PostfixExpressionOperatorAst::MemberAccess, Parser::parse_postfix_op_member_access),
            (PostfixExpressionOperatorAst::EarlyReturn, Parser::parse_postfix_op_early_return),
            (PostfixExpressionOperatorAst::NotKeyword, Parser::parse_postfix_op_not_keyword));
        Ok(p1)
    }

    pub fn parse_postfix_op_function_call(&mut self) -> ParserResult<PostfixExpressionOperatorFunctionCallAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_generic_arguments);
        let p2 = parse_once!(self, Parser::parse_function_call_arguments);
        let p3 = parse_optional!(self, Parser::parse_token_double_dot);
        Ok(PostfixExpressionOperatorFunctionCallAst::new(c1, p1, p2, p3))
    }

    pub fn parse_postfix_op_member_access(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_postfix_op_member_access_runtime,
            Parser::parse_postfix_op_member_access_static);
        Ok(p1)
    }

    pub fn parse_postfix_op_member_access_runtime(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let p2 = parse_alternate!(self,
            Parser::parse_identifier,
            Parser::parse_numeric_identifier);
        Ok(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2))
    }

    pub fn parse_postfix_op_member_access_static(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_colon);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2))
    }

    pub fn parse_postfix_op_early_return(&mut self) -> ParserResult<PostfixExpressionOperatorEarlyReturnAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_question_mark);
        Ok(PostfixExpressionOperatorEarlyReturnAst::new(c1, p1))
    }

    pub fn parse_postfix_op_not_keyword(&mut self) -> ParserResult<PostfixExpressionOperatorNotKeywordAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let p2 = parse_once!(self, Parser::parse_keyword_not);
        Ok(PostfixExpressionOperatorNotKeywordAst::new(c1, p1, p2))
    }

    pub fn parse_convention(&mut self) -> ParserResult<ConventionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (ConventionAst::Mut, Parser::parse_convention_mut),
            (ConventionAst::Ref, Parser::parse_convention_ref));
        Ok(p1)
    }

    pub fn parse_convention_mut(&mut self) -> ParserResult<ConventionMutAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_borrow);
        let p2 = parse_once!(self, Parser::parse_keyword_mut);
        Ok(ConventionMutAst::new(c1, p1, p2))
    }

    pub fn parse_convention_ref(&mut self) -> ParserResult<ConventionRefAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_borrow);
        Ok(ConventionRefAst::new(c1, p1))
    }

    pub fn parse_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_single);
        let p2 = parse_once!(self, Parser::parse_object_initializer_arguments);
        Ok(ObjectInitializerAst::new(c1, p1, p2))
    }

    pub fn parse_object_initializer_arguments(&mut self) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_object_initializer_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_object_initializer_argument(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (ObjectInitializerArgumentAst::Named, Parser::parse_object_initializer_argument_named),
            (ObjectInitializerArgumentAst::Unnamed, Parser::parse_object_initializer_argument_unnamed));
        Ok(p1)
    }

    pub fn parse_object_initializer_argument_unnamed(&mut self) -> ParserResult<ObjectInitializerArgumentUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_expression);
        Ok(ObjectInitializerArgumentUnnamedAst::new(c1, p1, p2))
    }

    pub fn parse_object_initializer_argument_named(&mut self) -> ParserResult<ObjectInitializerArgumentNamedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(ObjectInitializerArgumentNamedAst::new(c1, p1, p2, Box::new(p3)))
    }

    pub fn parse_type(&mut self) -> ParserResult<TypeAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_type_array,
            Parser::parse_type_tuple,
            Parser::parse_type_parenthesized,
            Parser::parse_type_binary_expression_precedence_level_1);
        Ok(p1)
    }

    pub fn parse_type_simple(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_type_unary_op_namespace, Parser::parse_nothing);
        let p2 = parse_once!(self, Parser::parse_type_single);
        p1.into_iter().rev().fold(Ok(p2), |acc, x| { Ok(TypeAst::Unary(TypeUnaryExpressionAst::new(c1, x, Box::from(acc?), ))) })
    }

    pub fn parse_type_binary_expression_precedence_level_n_rhs(&mut self, op: ParserRule<TokenAst>, rhs: ParserRule<TypeAst>) -> ParserResult<(TokenAst, TypeAst)> {
        let p1 = parse_once!(self, op);
        let p2 = parse_once!(self, rhs);
        Ok((p1, p2))
    }

    pub fn parse_type_binary_expression_precedence_level_n(&mut self, lhs: ParserRule<TypeAst>, op: ParserRule<TokenAst>, rhs: ParserRule<TypeAst>) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, lhs);
        let p2 = parse_optional!(self, |s| Parser::parse_type_binary_expression_precedence_level_n_rhs(s, op, rhs));
        Ok(if let Some(p2) = p2 { TypeAst::from(TypeBinaryExpressionAst::new(c1, Box::from(p1), p2.0, Box::new(p2.1))) } else { p1 })
    }

    pub fn parse_type_binary_expression_precedence_level_1(&mut self) -> ParserResult<TypeAst> {
        self.parse_type_binary_expression_precedence_level_n(
            Parser::parse_type_binary_expression_precedence_level_2,
            Parser::parse_type_binary_op_precedence_level_1,
            Parser::parse_type_binary_expression_precedence_level_1)
    }

    pub fn parse_type_binary_expression_precedence_level_2(&mut self) -> ParserResult<TypeAst> {
        self.parse_type_binary_expression_precedence_level_n(
            Parser::parse_type_postfix_expression,
            Parser::parse_type_binary_op_precedence_level_2,
            Parser::parse_type_binary_expression_precedence_level_2)
    }

    pub fn parse_type_postfix_expression(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_unary_expression);
        let p2 = parse_zero_or_more!(self, Parser::parse_type_postfix_op, Parser::parse_nothing);
        p2.into_iter().fold(Ok(p1), |acc, x| { Ok(TypeAst::Postfix(TypePostfixExpressionAst::new(c1, Box::from(acc?), x))) })
    }

    pub fn parse_type_unary_expression(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_type_unary_op_borrow);
        let mut p2 = parse_zero_or_more!(self, Parser::parse_type_unary_op_namespace, Parser::parse_nothing);
        let p3 = parse_once!(self, Parser::parse_type_single);
        if let Some(p1) = p1 {
            p2.insert(0, p1);
        }
        p2.into_iter().rev().fold(Ok(p3), |acc, x| { Ok(TypeAst::Unary(TypeUnaryExpressionAst::new(c1, x, Box::from(acc?), ))) })
    }

    pub fn parse_type_parenthesized(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_type);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(TypeAst::from(TypeParenthesizedExpressionAst::new(c1, p1, Box::from(p2), p3)))
    }

    pub fn parse_type_tuple(&mut self) -> ParserResult<TypeAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_type_tuple_1_items,
            Parser::parse_type_tuple_n_items);
        Ok(p1)
    }

    pub fn parse_type_array(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_once!(self, Parser::parse_type);
        let p3 = parse_once!(self, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p5 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(TypeAst::from(TypeArrayAst::new(c1, p1, p2, p3, p4, p5)))
    }

    pub fn parse_type_single(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_generic_identifier);
        Ok(TypeAst::Single(TypeSingleAst::new(c1, p1)))
    }

    pub fn parse_type_self(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_self_type);
        Ok(GenericIdentifierAst::new(c1, p1.metadata, None))
    }

    pub fn parse_type_tuple_1_items(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_type);
        let _a = parse_once!(self, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(TypeAst::from(TypeTupleAst::new(c1, p1, Vec::from([p2]), p3)))
    }

    pub fn parse_type_tuple_n_items(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_two_or_more!(self, Parser::parse_type, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(TypeAst::from(TypeTupleAst::new(c1, p1, p2, p3)))
    }

    pub fn parse_type_binary_op_precedence_level_1(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_or);
        Ok(p1)
    }

    pub fn parse_type_binary_op_precedence_level_2(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_and);
        Ok(p1)
    }

    pub fn parse_type_postfix_op(&mut self) -> ParserResult<TypePostfixExpressionOperatorAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (TypePostfixExpressionOperatorAst::Nested, Parser::parse_type_postfix_op_nested),
            (TypePostfixExpressionOperatorAst::Optional, Parser::parse_type_postfix_op_optional));
        Ok(p1)
    }

    pub fn parse_type_unary_op_borrow(&mut self) -> ParserResult<TypeUnaryExpressionOperatorAst> {
        let p1 = parse_once!(self, Parser::parse_convention);
        Ok(TypeUnaryExpressionOperatorAst::Borrow(p1))
    }

    pub fn parse_type_unary_op_namespace(&mut self) -> ParserResult<TypeUnaryExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_double_colon);
        Ok(TypeUnaryExpressionOperatorAst::Namespace(TypeUnaryExpressionOperatorNamespaceAst::new(c1, p1, p2)))
    }

    pub fn parse_type_postfix_op_nested(&mut self) -> ParserResult<TypePostfixExpressionOperatorNestedAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_colon);
        let p2 = parse_once!(self, Parser::parse_generic_identifier);
        Ok(TypePostfixExpressionOperatorNestedAst::new(c1, p1, p2))
    }

    pub fn parse_type_postfix_op_optional(&mut self) -> ParserResult<TypePostfixExpressionOperatorOptionalAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_question_mark);
        Ok(TypePostfixExpressionOperatorOptionalAst::new(c1, p1))
    }

    pub fn parse_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_identifier);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    pub fn parse_numeric_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    pub fn parse_upper_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_upper_identifier);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    pub fn parse_generic_identifier(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_arguments);
        Ok(GenericIdentifierAst::new(c1, p1.value, p2))
    }

    pub fn parse_literal(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LiteralAst::Float, Parser::parse_literal_float),
            (LiteralAst::Integer, Parser::parse_literal_integer),
            (LiteralAst::String, Parser::parse_literal_string),
            (LiteralAst::Tuple, Parser::parse_literal_tuple),
            (LiteralAst::Array0, Parser::parse_literal_array_empty),
            (LiteralAst::ArrayN, Parser::parse_literal_array),
            (LiteralAst::Boolean, Parser::parse_literal_boolean));
        Ok(p1)
    }

    pub fn parse_literal_float(&mut self) -> ParserResult<LiteralFloatAst> {
        let p1 = parse_once!(self, Parser::parse_literal_float_b10);
        Ok(p1)
    }

    pub fn parse_literal_integer(&mut self) -> ParserResult<LiteralIntegerAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_integer_b10,
            Parser::parse_literal_integer_b02,
            Parser::parse_literal_integer_b16);
        Ok(p1)
    }

    pub fn parse_literal_string(&mut self) -> ParserResult<LiteralStringAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_string);
        Ok(LiteralStringAst::new(c1, p1))
    }

    pub fn parse_literal_tuple(&mut self) -> ParserResult<LiteralTupleAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_tuple_0_items,
            Parser::parse_literal_tuple_1_items,
            Parser::parse_literal_tuple_n_items);
        Ok(p1)
    }

    pub fn parse_cmp_literal_tuple(&mut self) -> ParserResult<LiteralTupleAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_tuple_0_items,
            Parser::parse_cmp_literal_tuple_1_items,
            Parser::parse_cmp_literal_tuple_n_items);
        Ok(p1)
    }

    pub fn parse_literal_boolean(&mut self) -> ParserResult<LiteralBooleanAst> {
        let c1 = self.current_pos();
        let p1 = parse_alternate!(self,
            Parser::parse_keyword_true,
            Parser::parse_keyword_false);
         Ok(LiteralBooleanAst::new(c1, p1))
    }

    pub fn parse_literal_float_b10(&mut self) -> ParserResult<LiteralFloatAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p3 = parse_once!(self, Parser::parse_token_dot);
        let p4 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p5 = parse_optional!(self, Parser::parse_float_postfix_type);
        Ok(LiteralFloatAst::new(c1, p1, p2, p3, p4, p5.and_then(|x| Some(TypeAst::from(x)))))
    }

    pub fn parse_literal_integer_b10(&mut self) -> ParserResult<LiteralIntegerAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralIntegerAst::new(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    pub fn parse_literal_integer_b02(&mut self) -> ParserResult<LiteralIntegerAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_bin_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralIntegerAst::new(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    pub fn parse_literal_integer_b16(&mut self) -> ParserResult<LiteralIntegerAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_hex_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralIntegerAst::new(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    pub fn parse_numeric_prefix_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_sub,
            Parser::parse_token_add);
        Ok(p1)
    }

    pub fn parse_integer_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
        let _a = parse_once!(self, Parser::parse_token_underscore);
        let p1 = parse_alternate!(self,
            |x| Parser::parse_characters(x, "i8"),
            |x| Parser::parse_characters(x, "i16"),
            |x| Parser::parse_characters(x, "i32"),
            |x| Parser::parse_characters(x, "i64"),
            |x| Parser::parse_characters(x, "i128"),
            |x| Parser::parse_characters(x, "i256"),
            |x| Parser::parse_characters(x, "u8"),
            |x| Parser::parse_characters(x, "u16"),
            |x| Parser::parse_characters(x, "u32"),
            |x| Parser::parse_characters(x, "u64"),
            |x| Parser::parse_characters(x, "u128"),
            |x| Parser::parse_characters(x, "u256"),
            |x| Parser::parse_characters(x, "uz"));
        Ok(IdentifierAst::new(p1.pos, p1.metadata))
    }

    pub fn parse_float_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
        let _a = parse_once!(self, Parser::parse_token_underscore);
        let p1 = parse_alternate!(self,
            |x| Parser::parse_characters(x, "f8"),
            |x| Parser::parse_characters(x, "f16"),
            |x| Parser::parse_characters(x, "f32"),
            |x| Parser::parse_characters(x, "f64"),
            |x| Parser::parse_characters(x, "f128"),
            |x| Parser::parse_characters(x, "f256"));
        Ok(IdentifierAst::new(p1.pos, p1.metadata))
    }

    pub fn parse_literal_tuple_0_items(&mut self) -> ParserResult<LiteralTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralTupleAst::new(c1, p1, vec![], p2))
    }

    pub fn parse_literal_tuple_1_items(&mut self) -> ParserResult<LiteralTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_expression);
        let _a = parse_once!(self, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralTupleAst::new(c1, p1, vec![p2], p3))
    }

    pub fn parse_cmp_literal_tuple_1_items(&mut self) -> ParserResult<LiteralTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_cmp_value);
        let _a = parse_once!(self, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralTupleAst::new(c1, p1, vec![p2], p3))
    }

    pub fn parse_literal_tuple_n_items(&mut self) -> ParserResult<LiteralTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_two_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralTupleAst::new(c1, p1, p2, p3))
    }

    pub fn parse_cmp_literal_tuple_n_items(&mut self) -> ParserResult<LiteralTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_two_or_more!(self, Parser::parse_cmp_value, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralTupleAst::new(c1, p1, p2, p3))
    }

    pub fn parse_literal_array_empty(&mut self) -> ParserResult<LiteralArrayEmptyAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_once!(self, Parser::parse_type);
        let p3 = parse_once!(self, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p5 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralArrayEmptyAst::new(c1, p1, p2, p3, p4, p5))
    }

    pub fn parse_literal_array(&mut self) -> ParserResult<LiteralArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralArrayAst::new(c1, p1, p2, p3))
    }

    pub fn parse_cmp_literal_array(&mut self) -> ParserResult<LiteralArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_cmp_value, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralArrayAst::new(c1, p1, p2, p3))
    }

    pub fn parse_cmp_value(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Float(x))), Parser::parse_literal_float),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Integer(x))), Parser::parse_literal_integer),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::String(x))), Parser::parse_literal_string),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Tuple(x))), Parser::parse_cmp_literal_tuple),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Array0(x))), Parser::parse_literal_array_empty),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::ArrayN(x))), Parser::parse_cmp_literal_array),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Boolean(x))), Parser::parse_literal_boolean),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::ObjectInitializer(x)), Parser::parse_cmp_object_initializer),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Identifier(x)), Parser::parse_identifier));
        Ok(p1)
    }

    pub fn parse_cmp_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_simple);
        let p2 = parse_once!(self, Parser::parse_cmp_object_initializer_arguments);
        Ok(ObjectInitializerAst::new(c1, p1, p2))
    }

    pub fn parse_cmp_object_initializer_arguments(&mut self) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_cmp_object_initializer_argument_named, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3))
    }

    pub fn parse_cmp_object_initializer_argument_named(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_cmp_value);
        Ok(ObjectInitializerArgumentAst::Named(ObjectInitializerArgumentNamedAst::new(c1, p1, p2, Box::new(p3))))
    }

    pub fn parse_keyword(&mut self, keyword: Keywords, ast_keyword: TokenAstTokenType) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        for c in keyword.to_string().chars() {
            self.parse_character(c)?;
        }
        Ok(TokenAst::new(c1, ast_keyword, keyword.to_string()))
    }

    pub fn parse_token_left_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftCurlyBrace, TokenAstTokenType::TkLeftCurlyBrace));
        Ok(p1)
    }

    pub fn parse_token_right_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightCurlyBrace, TokenAstTokenType::TkRightCurlyBrace));
        Ok(p1)
    }

    pub fn parse_token_left_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftParenthesis, TokenAstTokenType::TkLeftParenthesis));
        Ok(p1)
    }

    pub fn parse_token_right_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightParenthesis, TokenAstTokenType::TkRightParenthesis));
        Ok(p1)
    }

    pub fn parse_token_left_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftSquareBracket, TokenAstTokenType::TkLeftSquareBracket));
        Ok(p1)
    }

    pub fn parse_token_right_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightSquareBracket, TokenAstTokenType::TkRightSquareBracket));
        Ok(p1)
    }

    pub fn parse_token_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkDot, TokenAstTokenType::TkDot));
        Ok(p1)
    }

    pub fn parse_token_comma(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkComma, TokenAstTokenType::TkComma));
        Ok(p1)
    }

    pub fn parse_token_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkColon, TokenAstTokenType::TkColon));
        Ok(p1)
    }

    pub fn parse_token_at(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAt, TokenAstTokenType::TkAt));
        Ok(p1)
    }

    pub fn parse_token_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::TkAssign));
        Ok(p1)
    }

    pub fn parse_token_newline(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNewLine, TokenAstTokenType::TkNewLine));
        Ok(p1)
    }

    pub fn parse_token_question_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkQuestionMark, TokenAstTokenType::TkQuestionMark));
        Ok(p1)
    }

    pub fn parse_token_borrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAmpersand, TokenAstTokenType::TkAmpersand));
        Ok(p1)
    }

    pub fn parse_token_lt(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLessThanSign, TokenAstTokenType::TkLessThanSign));
        Ok(p1)
    }

    pub fn parse_token_gt(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign, TokenAstTokenType::TkGreaterThanSign));
        Ok(p1)
    }

    pub fn parse_token_add(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPlusSign, TokenAstTokenType::TkPlusSign));
        Ok(p1)
    }

    pub fn parse_token_sub(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign, TokenAstTokenType::TkMinusSign));
        Ok(p1)
    }

    pub fn parse_token_mul(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::TkAsterisk));
        Ok(p1)
    }

    pub fn parse_token_div(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkForwardSlash, TokenAstTokenType::TkForwardSlash));
        Ok(p1)
    }

    pub fn parse_token_rem(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::TkPercentSign));
        Ok(p1)
    }

    pub fn parse_token_underscore(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore, TokenAstTokenType::TkUnderscore));
        Ok(p1)
    }

    pub fn parse_token_speech_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkSpeechMark, TokenAstTokenType::TkSpeechMark));
        Ok(p1)
    }

    pub fn parse_token_double_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkColon, TokenAstTokenType::TkColon));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkColon, TokenAstTokenType::TkColon));
        Ok(p1)
    }

    pub fn parse_token_double_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkDot, TokenAstTokenType::TkDot));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkDot, TokenAstTokenType::TkDot));
        Ok(p1)
    }

    pub fn parse_token_rightward_arrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign, TokenAstTokenType::TkRightwardsArrow));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_eq(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::TkEquals));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_ne(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkExclamationMark, TokenAstTokenType::TkNotEquals));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_le(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLessThanSign, TokenAstTokenType::TkLessThanOrEqualSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_ge(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign, TokenAstTokenType::TkGreaterThanOrEqualSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_add_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPlusSign, TokenAstTokenType::TkPlusEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_sub_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign, TokenAstTokenType::TkMinusEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_mul_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::TkAsteriskEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_div_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkForwardSlash, TokenAstTokenType::TkForwardSlashEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_rem_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::TkPercentEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_mod_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::TkDoublePercentEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::NoToken));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_exp_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::TkExponentEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::NoToken));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_mod(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::TkDoublePercentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_token_exp(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::TkExponentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_nothing(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::NoToken, TokenAstTokenType::NoToken));
        Ok(p1)
    }

    pub fn parse_keyword_cls(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cls, TokenAstTokenType::KwCls));
        Ok(p1)
    }

    pub fn parse_keyword_sup(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Sup, TokenAstTokenType::KwSup));
        Ok(p1)
    }

    pub fn parse_keyword_ext(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Ext, TokenAstTokenType::KwExt));
        Ok(p1)
    }

    pub fn parse_keyword_fun(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Fun, TokenAstTokenType::KwFun));
        Ok(p1)
    }

    pub fn parse_keyword_cor(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cor, TokenAstTokenType::KwCor));
        Ok(p1)
    }

    pub fn parse_keyword_use(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Use, TokenAstTokenType::KwUse));
        Ok(p1)
    }

    pub fn parse_keyword_let(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Let, TokenAstTokenType::KwLet));
        Ok(p1)
    }

    pub fn parse_keyword_mut(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Mut, TokenAstTokenType::KwMut));
        Ok(p1)
    }

    pub fn parse_keyword_cmp(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cmp, TokenAstTokenType::KwCmp));
        Ok(p1)
    }

    pub fn parse_keyword_where(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Where, TokenAstTokenType::KwWhere));
        Ok(p1)
    }

    pub fn parse_keyword_self_val(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::SelfVal, TokenAstTokenType::KwSelfVal));
        Ok(p1)
    }

    pub fn parse_keyword_self_type(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::SelfType, TokenAstTokenType::KwSelfType));
        Ok(p1)
    }

    pub fn parse_keyword_case(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Case, TokenAstTokenType::KwCase));
        Ok(p1)
    }

    pub fn parse_keyword_loop(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Loop, TokenAstTokenType::KwLoop));
        Ok(p1)
    }

    pub fn parse_keyword_gen(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Gen, TokenAstTokenType::KwGen));
        Ok(p1)
    }

    pub fn parse_keyword_with(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::With, TokenAstTokenType::KwWith));
        Ok(p1)
    }

    pub fn parse_keyword_ret(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Ret, TokenAstTokenType::KwRet));
        Ok(p1)
    }

    pub fn parse_keyword_skip(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Skip, TokenAstTokenType::KwSkip));
        Ok(p1)
    }

    pub fn parse_keyword_exit(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Exit, TokenAstTokenType::KwExit));
        Ok(p1)
    }

    pub fn parse_keyword_else(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Else, TokenAstTokenType::KwElse));
        Ok(p1)
    }

    pub fn parse_keyword_false(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::False, TokenAstTokenType::KwFalse));
        Ok(p1)
    }

    pub fn parse_keyword_true(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::True, TokenAstTokenType::KwTrue));
        Ok(p1)
    }

    pub fn parse_keyword_of(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Of, TokenAstTokenType::KwOf));
        Ok(p1)
    }

    pub fn parse_keyword_in(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::In, TokenAstTokenType::KwIn));
        Ok(p1)
    }

    pub fn parse_keyword_as(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::As, TokenAstTokenType::KwAs));
        Ok(p1)
    }

    pub fn parse_keyword_is(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Is, TokenAstTokenType::KwIs));
        Ok(p1)
    }

    pub fn parse_keyword_and(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::And, TokenAstTokenType::KwAnd));
        Ok(p1)
    }

    pub fn parse_keyword_or(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Or, TokenAstTokenType::KwOr));
        Ok(p1)
    }

    pub fn parse_keyword_not(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Not, TokenAstTokenType::KwNot));
        Ok(p1)
    }

    pub fn parse_keyword_async(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Async, TokenAstTokenType::KwAsync));
        Ok(p1)
    }

    pub fn parse_lexeme_dec_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();

        parse_once!(self, Parser::parse_nothing);
        match self.tokens[self.current_pos()] {
            TokenType::TkNumber(num) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num), TokenAstTokenType::TkNumber));
                integer.push(num);
            }
            _ => return Err(SyntaxError::new(c1, "Expected decimal integer".to_string())),
        }

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num), TokenAstTokenType::TkNumber));
            integer.push(p1.metadata.as_bytes()[0] as char);
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected decimal integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, integer))
        }
    }

    pub fn parse_lexeme_bin_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber('0'), TokenAstTokenType::NoToken));
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter('b'), TokenAstTokenType::NoToken));

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num), TokenAstTokenType::TkNumber));
            integer.push(p1.metadata.as_bytes()[0] as char);
            if !num.is_digit(2) {
                return Err(SyntaxError::new(c1, "Expected binary integer".to_string()));
            }
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected binary integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, integer))
        }
    }

    pub fn parse_lexeme_hex_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber('0'), TokenAstTokenType::NoToken));
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter('x'), TokenAstTokenType::NoToken));

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num), TokenAstTokenType::TkNumber));
            integer.push(p1.metadata.as_bytes()[0] as char);
            if !num.is_digit(16) {
                return Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()));
            }
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, integer))
        }
    }

    pub fn parse_lexeme_string(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        parse_once!(self, Parser::parse_token_speech_mark);

        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }

        parse_once!(self, Parser::parse_token_speech_mark);
        Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, identifier))
    }

    pub fn parse_lexeme_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        parse_once!(self, Parser::parse_nothing);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) if string.is_lowercase() => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
                identifier.push(string);
            },
            _ => return Err(SyntaxError::new(c1, "Expected identifier".to_string())),
        }

        loop {
            match self.tokens[self.current_pos()] {
                TokenType::TkCharacter(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
                    identifier.push(string);
                },
                TokenType::TkNumber(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string), TokenAstTokenType::TkNumber));
                    identifier.push(string);
                },
                TokenType::TkUnderscore => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore, TokenAstTokenType::TkUnderscore));
                    identifier.push('_');
                },
                _ => break,
            }
        }

        if identifier.is_empty() {
            return Err(SyntaxError::new(c1, "Expected identifier".to_string()));
        }

        Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, identifier))
    }

    pub fn parse_lexeme_upper_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        parse_once!(self, Parser::parse_nothing);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) if string.is_uppercase() => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
                identifier.push(string);
            },
            _ => return Err(SyntaxError::new(c1, "Expected upper identifier".to_string())),
        }

        loop {
            match self.tokens[self.current_pos()] {
                TokenType::TkCharacter(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
                    identifier.push(string);
                },
                TokenType::TkNumber(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string), TokenAstTokenType::TkNumber));
                    identifier.push(string);
                },
                TokenType::TkUnderscore => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore, TokenAstTokenType::TkUnderscore));
                    identifier.push('_');
                },
                _ => break,
            }
        }

        if identifier.is_empty() {
            return Err(SyntaxError::new(c1, "Expected upper identifier".to_string()));
        }

        if (identifier.as_bytes()[0] as char).is_lowercase() {
            return Err(SyntaxError::new(c1, "Expected upper identifier".to_string()));
        }

        Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, identifier))
    }

    pub fn parse_characters(&mut self, characters: &str) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        for c in characters.chars() {
            let p1 = parse_once!(self, |x| Parser::parse_character(x, c));
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }

        if identifier != characters {
            return Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", characters, identifier)));
        }

        Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, "".to_string()))
    }

    pub fn parse_character(&mut self, character: char) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();

        parse_once!(self, Parser::parse_nothing);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string), TokenAstTokenType::TkString));
                if string != character {
                    Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", character, string)))
                }
                else {
                    Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, string.to_string()))
                }
            },
            TokenType::TkNumber(string) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string), TokenAstTokenType::TkNumber));
                if string != character {
                    Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", character, string)))
                }
                else {
                    Ok(TokenAst::new(c1, TokenAstTokenType::NoToken, string.to_string()))
                }
            },
            _ => Err(SyntaxError::new(c1, "Expected upper identifier".to_string())),
        }
    }

    pub fn parse_eof(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::EndOfFile, TokenAstTokenType::EndOfFile));
        Ok(p1)
    }

    pub fn parse_token_primitive(&mut self, token_type: TokenType, ast_token_type: TokenAstTokenType) -> ParserResult<TokenAst> {
        if self.index > self.token_len {
            let new_error = format!("Expected token {:?} but found end of file", token_type);
            self.store_error(self.current_pos(), new_error);
            return Err(self.error.clone());
        }

        if token_type != TokenType::TkNewLine {
            while self.tokens[self.index] == TokenType::TkNewLine || self.tokens[self.index] == TokenType::TkWhitespace {
                self.index += 1;
            }
        }
        if token_type == TokenType::TkNewLine {
            while self.tokens[self.index] == TokenType::TkWhitespace {
                self.index += 1;
            }
        }

        if token_type == TokenType::NoToken {
            return Ok(TokenAst::new(self.current_pos(), TokenAstTokenType::NoToken, "".to_string()));
        }

        if self.tokens[self.index] != token_type {
            if self.error.get_pos() == self.index {
                self.error.add_expected_token(token_type);
                return Err(self.error.clone());
            }

            let new_error = format!("Expected '£' got '{}'", self.tokens[self.index]);
            if self.store_error(self.index, new_error) {
                self.error.add_expected_token(token_type);
            }
            return Err(self.error.clone());
        }

        let token_character = match self.tokens[self.index] {
            TokenType::TkCharacter(c) => String::from(c),
            TokenType::TkNumber(c) => String::from(c),
            _ => "".to_string(),
        };

        let r = TokenAst::new(self.current_pos(), ast_token_type, token_character);
        self.index += 1;
        Ok(r)
    }
}
