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
use crate::spp::asts::global_constant_ast::GlobalConstantAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::inner_scope_ast::InnerScopeAst;
use crate::spp::asts::let_statement_ast::LetStatementAst;
use crate::spp::asts::literal_ast::LiteralAst;
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
use crate::spp::asts::pin_statement_ast::PinStatementAst;
use crate::spp::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::spp::asts::postfix_expression_operator_ast::PostfixExpressionOperatorAst;
use crate::spp::asts::postfix_expression_operator_early_return_ast::PostfixExpressionOperatorEarlyReturnAst;
use crate::spp::asts::postfix_expression_operator_function_call_ast::PostfixExpressionOperatorFunctionCallAst;
use crate::spp::asts::postfix_expression_operator_member_access_ast::PostfixExpressionOperatorMemberAccessAst;
use crate::spp::asts::postfix_expression_operator_not_keyword_ast::PostfixExpressionOperatorNotKeywordAst;
use crate::spp::asts::postfix_expression_operator_step_keyword_ast::PostfixExpressionOperatorStepKeywordAst;
use crate::spp::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::spp::asts::rel_statement_ast::RelStatementAst;
use crate::spp::asts::ret_statement_ast::RetStatementAst;
use crate::spp::asts::statement_ast::StatementAst;
use crate::spp::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::spp::asts::sup_implementation_ast::SupImplementationAst;
use crate::spp::asts::sup_member_ast::SupMemberAst;
use crate::spp::asts::sup_method_prototype_ast::SupMethodPrototypeAst;
use crate::spp::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::spp::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::spp::asts::sup_use_statement_ast::SupUseStatementAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_optional_ast::TypeOptionalAst;
use crate::spp::asts::type_tuple_ast::TypeTupleAst;
use crate::spp::asts::type_variant_ast::TypeVariantAst;
use crate::spp::asts::unary_expression_ast::UnaryExpressionAst;
use crate::spp::asts::unary_expression_operator_ast::UnaryExpressionOperatorAst;
use crate::spp::asts::unary_expression_operator_async_ast::UnaryExpressionOperatorAsyncAst;
use crate::spp::asts::use_statement_ast::UseStatementAst;
use crate::spp::asts::where_block_ast::WhereBlockAst;
use crate::spp::asts::where_constraints_ast::WhereConstraintsAst;
use crate::spp::asts::where_constraints_group_ast::WhereConstraintsGroupAst;
use crate::spp::asts::with_expression_alias_ast::WithExpressionAliasAst;
use crate::spp::asts::with_expression_ast::WithExpressionAst;
use crate::spp::lexer::token::{Keywords, TokenStream, TokenType};
use crate::spp::parser::parser_error::SyntaxError;

type ParserResult<T> = Result<T, SyntaxError>;
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

    fn parse_root(&mut self) -> ParserResult<ModulePrototypeAst> {
        let p1 = parse_once!(self, Parser::parse_module_prototype);
        let _a = parse_once!(self, Parser::parse_eof);
        Ok(p1)
    }

    fn parse_module_prototype(&mut self) -> ParserResult<ModulePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_module_implementation);
        Ok(ModulePrototypeAst::new(c1, p1))
    }

    fn parse_module_implementation(&mut self) -> ParserResult<ModuleImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_module_member, Parser::parse_token_no_token);
        Ok(ModuleImplementationAst::new(c1, p1))
    }

    fn parse_module_member(&mut self) -> ParserResult<ModuleMemberAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (ModuleMemberAst::Function, Parser::parse_function_prototype),
            (ModuleMemberAst::GlobalConst, Parser::parse_comp_global_constant),
            (ModuleMemberAst::UseStatement, Parser::parse_global_use_statement),
            (ModuleMemberAst::Class, Parser::parse_class_prototype),
            (ModuleMemberAst::SupExtension, Parser::parse_sup_prototype_extension),
            (ModuleMemberAst::SupFunctions, Parser::parse_sup_prototype_functions));
        Ok(p1)
    }

    fn parse_class_prototype(&mut self) -> ParserResult<ClassPrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_keyword_cls);
        let p3 = parse_once!(self, Parser::parse_upper_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_optional!(self, Parser::parse_where_block);
        let p6 = parse_once!(self, Parser::parse_class_implementation);
        Ok(ClassPrototypeAst::new(c1, p1, p2, TypeAst::from(p3), p4, p5, p6))
    }

    fn parse_class_implementation(&mut self) -> ParserResult<ClassImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_class_member, Parser::parse_token_no_token);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(ClassImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_class_member(&mut self) -> ParserResult<ClassMemberAst> {
        let p1 = parse_once!(self, Parser::parse_class_attribute);
        Ok(ClassMemberAst::Attr(p1))
    }

    fn parse_class_attribute(&mut self) -> ParserResult<ClassAttributeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(ClassAttributeAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_sup_prototype_extension(&mut self) -> ParserResult<SupPrototypeExtensionAst> {
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

    fn parse_sup_prototype_functions(&mut self) -> ParserResult<SupPrototypeFunctionsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_sup);
        let p2 = parse_optional!(self, Parser::parse_generic_parameters);
        let p3 = parse_once!(self, Parser::parse_type);
        let p4 = parse_optional!(self, Parser::parse_where_block);
        let p5 = parse_once!(self, Parser::parse_sup_implementation);
        Ok(SupPrototypeFunctionsAst::new(c1, p1, p2, p3, p4, p5))
    }

    fn parse_sup_implementation(&mut self) -> ParserResult<SupImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_sup_member, Parser::parse_token_no_token);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(SupImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_sup_member(&mut self) -> ParserResult<SupMemberAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (SupMemberAst::Method, Parser::parse_sup_method_prototype),
            (SupMemberAst::Typedef, Parser::parse_sup_use_statement));
        Ok(p1)
    }

    fn parse_sup_method_prototype(&mut self) -> ParserResult<SupMethodPrototypeAst> {
        let p1 = parse_once!(self, Parser::parse_function_prototype);
        Ok(p1)
    }

    fn parse_sup_use_statement(&mut self) -> ParserResult<SupUseStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let mut p2 = parse_once!(self, Parser::parse_use_statement);
        p2.annotations = p1;
        Ok(p2)
    }

    fn parse_function_prototype(&mut self) -> ParserResult<FunctionPrototypeAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (FunctionPrototypeAst::Subroutine, Parser::parse_subroutine_prototype),
            (FunctionPrototypeAst::Coroutine, Parser::parse_coroutine_prototype));
        Ok(p1)
    }

    fn parse_subroutine_prototype(&mut self) -> ParserResult<SubroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_keyword_fun);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_once!(self, Parser::parse_function_parameters);
        let p6 = parse_once!(self, Parser::parse_token_rightward_arrow);
        let p7 = parse_once!(self, Parser::parse_type);
        let p8 = parse_optional!(self, Parser::parse_where_block);
        let p9 = parse_once!(self, Parser::parse_function_implementation);
        Ok(SubroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9)))
    }

    fn parse_coroutine_prototype(&mut self) -> ParserResult<CoroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_keyword_cor);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_optional!(self, Parser::parse_generic_parameters);
        let p5 = parse_once!(self, Parser::parse_function_parameters);
        let p6 = parse_once!(self, Parser::parse_token_rightward_arrow);
        let p7 = parse_once!(self, Parser::parse_type);
        let p8 = parse_optional!(self, Parser::parse_where_block);
        let p9 = parse_once!(self, Parser::parse_function_implementation);
        Ok(CoroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9)))
    }

    fn parse_function_implementation(&mut self) -> ParserResult<FunctionImplementationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_member, Parser::parse_token_no_token);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(FunctionImplementationAst::new(c1, p1, p2, p3))
    }

    fn parse_function_member(&mut self) -> ParserResult<FunctionMemberAst> {
        let p1 = parse_once!(self, Parser::parse_statement);
        Ok(p1)
    }

    fn parse_function_call_arguments(&mut self) -> ParserResult<FunctionCallArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_call_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(FunctionCallArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_function_call_argument(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_function_call_argument_named,
            Parser::parse_function_call_argument_unnamed);
        Ok(p1)
    }

    fn parse_function_call_argument_unnamed(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_convention);
        let p2 = parse_optional!(self, Parser::parse_token_double_dot);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionCallArgumentAst::new_unnamed_argument(c1, p1, p2, p3))
    }

    fn parse_function_call_argument_named(&mut self) -> ParserResult<FunctionCallArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_convention);
        let p4 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionCallArgumentAst::new_named_argument(c1, p1, p2, p3, p4))
    }

    fn parse_function_parameters(&mut self) -> ParserResult<FunctionParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_function_parameter, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(FunctionParameterGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_function_parameter(&mut self) -> ParserResult<FunctionParameterAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_function_parameter_variadic,
            Parser::parse_function_parameter_optional,
            Parser::parse_function_parameter_required,
            Parser::parse_function_parameter_self);
        Ok(p1)
    }

    fn parse_function_parameter_self(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_convention);
        let p3 = parse_once!(self, Parser::parse_self_identifier);
        Ok(FunctionParameterAst::new_self(c1, p1, p2, p3))
    }

    fn parse_function_parameter_required(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_convention);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(FunctionParameterAst::new_required(c1, p1, p2, p3, p4))
    }

    fn parse_function_parameter_optional(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_convention);
        let p4 = parse_once!(self, Parser::parse_type);
        let p5 = parse_once!(self, Parser::parse_token_assign);
        let p6 = parse_once!(self, Parser::parse_expression);
        Ok(FunctionParameterAst::new_optional(
            c1, p1, p2, p3, p4, p5, p6,
        ))
    }

    fn parse_function_parameter_variadic(&mut self) -> ParserResult<FunctionParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_convention);
        let p5 = parse_once!(self, Parser::parse_type);
        Ok(FunctionParameterAst::new_variadic(c1, p1, p2, p3, p4, p5))
    }

    fn parse_generic_arguments(&mut self) -> ParserResult<GenericArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_zero_or_more!(self, Parser::parse_generic_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(GenericArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_generic_argument(&mut self) -> ParserResult<GenericArgumentAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_generic_argument_type_named,
            Parser::parse_generic_argument_type_unnamed,
            Parser::parse_generic_argument_comp_named,
            Parser::parse_generic_argument_comp_unnamed);
        Ok(p1)
    }

    fn parse_generic_argument_type_named(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_type);
        Ok(GenericArgumentAst::new_type_named(c1, TypeAst::from(p1), p2, p3))
    }

    fn parse_generic_argument_type_unnamed(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type);
        Ok(GenericArgumentAst::new_type_unnamed(c1, p1))
    }

    fn parse_generic_argument_comp_named(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_comp_value);
        Ok(GenericArgumentAst::new_comp_named(c1, TypeAst::from(p1), p2, p3))
    }

    fn parse_generic_argument_comp_unnamed(&mut self) -> ParserResult<GenericArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_comp_value);
        Ok(GenericArgumentAst::new_comp_unnamed(c1, p1))
    }

    fn parse_generic_parameters(&mut self) -> ParserResult<GenericParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_zero_or_more!(self, Parser::parse_generic_parameter, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(GenericParameterGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_generic_parameter(&mut self) -> ParserResult<GenericParameterAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_generic_parameter_comp_variadic,
            Parser::parse_generic_parameter_comp_optional,
            Parser::parse_generic_parameter_comp_required,
            Parser::parse_generic_parameter_type_variadic,
            Parser::parse_generic_parameter_type_optional,
            Parser::parse_generic_parameter_type_required);
        Ok(p1)
    }

    fn parse_generic_parameter_type_required(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        Ok(GenericParameterAst::new_type_required(c1, TypeAst::from(p1), p2))
    }

    fn parse_generic_parameter_type_optional(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        let p3 = parse_once!(self, Parser::parse_token_assign);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterAst::new_type_optional(c1, TypeAst::from(p1), p2, p3, p4))
    }

    fn parse_generic_parameter_type_variadic(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_upper_identifier);
        let p3 = parse_optional!(self, Parser::parse_generic_inline_constraints);
        Ok(GenericParameterAst::new_type_variadic(c1, p1, TypeAst::from(p2), p3))
    }

    fn parse_generic_parameter_comp_required(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterAst::new_comp_required(c1, p1, TypeAst::from(p2), p3, p4))
    }

    fn parse_generic_parameter_comp_optional(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        let p5 = parse_once!(self, Parser::parse_token_assign);
        let p6 = parse_once!(self, Parser::parse_comp_value);
        Ok(GenericParameterAst::new_comp_optional(c1, p1, TypeAst::from(p2), p3, p4, p5, p6))
    }

    fn parse_generic_parameter_comp_variadic(&mut self) -> ParserResult<GenericParameterAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_cmp);
        let p2 = parse_once!(self, Parser::parse_token_double_dot);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_once!(self, Parser::parse_token_colon);
        let p5 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterAst::new_comp_variadic(c1, p1, p2, TypeAst::from(p3), p4, p5))
    }

    fn parse_generic_inline_constraints(&mut self) -> ParserResult<GenericParameterConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_colon);
        let p2 = parse_once!(self, Parser::parse_type);
        Ok(GenericParameterConstraintsAst::new(c1, p1, p2))
    }

    fn parse_where_block(&mut self) -> ParserResult<WhereBlockAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_where);
        let p2 = parse_once!(self, Parser::parse_where_block_constraints_group);
        Ok(WhereBlockAst::new(c1, p1, p2))
    }

    fn parse_where_block_constraints_group(&mut self) -> ParserResult<WhereConstraintsGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_where_block_constraints, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(WhereConstraintsGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_where_block_constraints(&mut self) -> ParserResult<WhereConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_type, Parser::parse_token_comma);
        let p2 = parse_once!(self, Parser::parse_token_colon);
        let p3 = parse_once!(self, Parser::parse_type);
        Ok(WhereConstraintsAst::new(c1, p1, p2, p3))
    }

    fn parse_annotation(&mut self) -> ParserResult<AnnotationAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_at);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(AnnotationAst::new(c1, p1, p2))
    }

    fn parse_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_once!(self, Parser::parse_binary_expression_precedence_level_1);
        Ok(p1)
    }

    fn parse_binary_expression_precedence_level_n_rhs<T: ToBinaryExpression + 'static>(&mut self, op: ParserRule<TokenAst>, rhs: ParserRule<T>) -> ParserResult<(TokenAst, T)> {
        let p1 = parse_once!(self, op);
        let p2 = parse_once!(self, rhs);
        Ok((p1, p2))
    }

    fn parse_binary_expression_precedence_level_n<T: ToBinaryExpression + 'static>(&mut self, lhs: ParserRule<ExpressionAst>, op: ParserRule<TokenAst>, rhs: ParserRule<T>) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, lhs);
        let p2 = parse_optional!(self, |s| Parser::parse_binary_expression_precedence_level_n_rhs(s, op, rhs));
        Ok(if let Some(p2) = p2 { T::to_binary_expression(c1, p1, p2.0, p2.1) } else { p1 })
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
        let p1 = parse_zero_or_more!(self, Parser::parse_unary_op, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_postfix_expression);
        p1.into_iter().rev().fold(Ok(p2), |acc, x| { Ok(ExpressionAst::Unary(UnaryExpressionAst::new(c1, x, Box::from(acc?), ))) })
    }

    fn parse_postfix_expression(&mut self) -> ParserResult<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_primary_expression);
        let p2 = parse_zero_or_more!(self, Parser::parse_postfix_op, Parser::parse_token_no_token);
        p2.into_iter().fold(Ok(p1), |acc, x| { Ok(ExpressionAst::Postfix(PostfixExpressionAst::new(c1, Box::from(acc?), x, ))) })
    }

    fn parse_primary_expression(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::ObjectInitializer(x)), Parser::parse_object_initializer),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Parenthesized(x)), Parser::parse_parenthesized_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Type(x)), Parser::parse_type),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Identifier(x)), Parser::parse_identifier),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Case(x)), Parser::parse_case_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Loop(x)), Parser::parse_loop_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Gen(x)), Parser::parse_gen_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::With(x)), Parser::parse_with_expression),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::InnerScope(x)), Parser::parse_inner_scope),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::SelfIdentifier(x)), Parser::parse_self_identifier),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Fold(x)), Parser::parse_fold_expression));
        Ok(p1)
    }

    fn parse_parenthesized_expression(&mut self) -> ParserResult<ParenthesizedExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ParenthesizedExpressionAst::new(c1, p1, Box::from(p2), p3))
    }

    fn parse_self_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_self_val);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    fn parse_fold_expression(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        Ok(p1)
    }

    fn parse_case_expression(&mut self) -> ParserResult<CaseExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_case_expression_patterns,
            Parser::parse_case_expression_simple);
        Ok(p1)
    }

    fn parse_case_expression_patterns(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_case);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_keyword_of);
        let p4 = parse_one_or_more!(self, Parser::parse_case_expression_branch, Parser::parse_token_no_token);
        Ok(CaseExpressionAst::new(c1, p1, Box::from(p2), p3, p4))
    }

    fn parse_case_expression_simple(&mut self) -> ParserResult<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_case);
        let p2 = parse_once!(self, Parser::parse_expression);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        let p4 = parse_zero_or_more!(self, Parser::parse_case_expression_branch_simple, Parser::parse_token_no_token);
        Ok(CaseExpressionAst::new_from_simple(c1, p1, Box::from(p2), p3, p4))
    }

    fn parse_loop_expression(&mut self) -> ParserResult<LoopExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_loop);
        let p2 = parse_once!(self, Parser::parse_loop_expression_condition);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        let p4 = parse_optional!(self, Parser::parse_loop_else_statement);
        Ok(LoopExpressionAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_loop_expression_condition(&mut self) -> ParserResult<LoopConditionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_loop_expression_condition_iterable,
            Parser::parse_loop_expression_condition_boolean);
        Ok(p1)
    }

    fn parse_loop_expression_condition_boolean(&mut self) -> ParserResult<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_expression);
        Ok(LoopConditionAst::new_boolean(c1, Box::new(p1)))
    }

    fn parse_loop_expression_condition_iterable(&mut self) -> ParserResult<LoopConditionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_local_variable);
        let p2 = parse_once!(self, Parser::parse_keyword_in);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(LoopConditionAst::new_iterable(c1, p1, p2, Box::new(p3)))
    }

    fn parse_loop_else_statement(&mut self) -> ParserResult<LoopElseStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        let p2 = parse_once!(self, Parser::parse_inner_scope);
        Ok(LoopElseStatementAst::new(c1, p1, p2))
    }

    fn parse_gen_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_gen_expression_unroll,
            Parser::parse_gen_expression_normal);
        Ok(p1)
    }

    fn parse_gen_expression_normal(&mut self) -> ParserResult<GenExpressionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_gen_expression_normal_with_expression,
            Parser::parse_gen_expression_normal_no_expression);
        Ok(p1)
    }

    fn parse_gen_expression_normal_no_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        Ok(GenExpressionAst::new(c1, p1, None, ConventionAst::Mov { pos: c1 }, None, ))
    }

    fn parse_gen_expression_normal_with_expression(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        let p2 = parse_once!(self, Parser::parse_convention);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(GenExpressionAst::new(c1, p1, None, p2, Some(Box::new(p3))))
    }

    fn parse_gen_expression_unroll(&mut self) -> ParserResult<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_gen);
        let p2 = parse_once!(self, Parser::parse_keyword_with);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(GenExpressionAst::new(c1, p1, Some(p2), ConventionAst::Mov { pos: c1 }, Some(Box::new(p3))))
    }

    fn parse_with_expression(&mut self) -> ParserResult<WithExpressionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_with);
        let p2 = parse_optional!(self, Parser::parse_with_expression_lhs_alias);
        let p3 = parse_once!(self, Parser::parse_expression);
        let p4 = parse_once!(self, Parser::parse_inner_scope);
        Ok(WithExpressionAst::new(c1, p1, p2, Box::new(p3), p4))
    }

    fn parse_with_expression_lhs_alias(&mut self) -> ParserResult<WithExpressionAliasAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_as);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        Ok(WithExpressionAliasAst::new(c1, p1, p2))
    }

    fn parse_ret_statement(&mut self) -> ParserResult<RetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_ret);
        let p2 = parse_optional!(self, Parser::parse_expression);
        Ok(RetStatementAst::new(c1, p1, p2))
    }

    fn parse_exit_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_keyword_exit, Parser::parse_token_no_token);
        let p2 = parse_optional!(self, Parser::parse_exit_statement_final_action);
        Ok(LoopControlFlowStatementAst::new(c1, p1, p2))
    }

    fn parse_exit_statement_final_action(&mut self) -> ParserResult<LoopControlFlowStatementFinalPartAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LoopControlFlowStatementFinalPartAst::Skip, Parser::parse_keyword_skip),
            (LoopControlFlowStatementFinalPartAst::Expression, Parser::parse_expression));
        Ok(p1)
    }

    fn parse_skip_statement(&mut self) -> ParserResult<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_keyword_skip, Parser::parse_token_no_token);
        Ok(LoopControlFlowStatementAst::new(c1, p1, None))
    }

    fn parse_pin_statement(&mut self) -> ParserResult<PinStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_pin);
        let p2 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        Ok(PinStatementAst::new(c1, p1, p2))
    }

    fn parse_rel_statement(&mut self) -> ParserResult<RelStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_rel);
        let p2 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        Ok(RelStatementAst::new(c1, p1, p2))
    }

    fn parse_inner_scope(&mut self) -> ParserResult<InnerScopeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_zero_or_more!(self, Parser::parse_statement, Parser::parse_token_no_token);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(InnerScopeAst::new(c1, p1, p2, p3))
    }

    fn parse_statement(&mut self) -> ParserResult<StatementAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (StatementAst::Use, Parser::parse_use_statement),
            (StatementAst::Let, Parser::parse_let_statement),
            (StatementAst::Ret, Parser::parse_ret_statement),
            (StatementAst::LoopControlFlow , Parser::parse_exit_statement),
            (StatementAst::LoopControlFlow , Parser::parse_skip_statement),
            (StatementAst::Pin, Parser::parse_pin_statement),
            (StatementAst::Rel, Parser::parse_rel_statement),
            (StatementAst::Assignment, Parser::parse_assignment_statement),
            (StatementAst::Expression, Parser::parse_expression));
        Ok(p1)
    }

    fn parse_global_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let mut p2 = parse_once!(self, Parser::parse_use_statement);
        p2.annotations = p1;
        Ok(p2)
    }

    fn parse_use_statement(&mut self) -> ParserResult<UseStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_use);
        let p2 = parse_once!(self, Parser::parse_upper_identifier);
        let p3 = parse_optional!(self, Parser::parse_generic_parameters);
        let p4 = parse_once!(self, Parser::parse_token_assign);
        let p5 = parse_once!(self, Parser::parse_type);
        Ok(UseStatementAst::new(c1, vec![], p1, TypeAst::from(p2), p3, p4, p5))
    }

    fn parse_comp_global_constant(&mut self) -> ParserResult<GlobalConstantAst> {
        let c1 = self.current_pos();
        let p1 = parse_zero_or_more!(self, Parser::parse_annotation, Parser::parse_token_no_token);
        let p2 = parse_once!(self, Parser::parse_keyword_cmp);
        let p3 = parse_once!(self, Parser::parse_identifier);
        let p4 = parse_once!(self, Parser::parse_token_colon);
        let p5 = parse_once!(self, Parser::parse_type);
        let p6 = parse_once!(self, Parser::parse_token_assign);
        let p7 = parse_once!(self, Parser::parse_comp_value);
        Ok(GlobalConstantAst::new(c1, p1, p2, p3, p4, p5, p6, p7))
    }

    fn parse_let_statement(&mut self) -> ParserResult<LetStatementAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_let_statement_initialized,
            Parser::parse_let_statement_uninitialized);
        Ok(p1)
    }

    fn parse_let_statement_initialized(&mut self) -> ParserResult<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_let);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_assign);
        let p4 = parse_once!(self, Parser::parse_expression);
        Ok(LetStatementAst::new_initialized(c1, p1, p2, p3, p4))
    }

    fn parse_let_statement_uninitialized(&mut self) -> ParserResult<LetStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_let);
        let p2 = parse_once!(self, Parser::parse_local_variable);
        let p3 = parse_once!(self, Parser::parse_token_colon);
        let p4 = parse_once!(self, Parser::parse_type);
        Ok(LetStatementAst::new_uninitialized(c1, p1, p2, p3, p4))
    }

    fn parse_local_variable(&mut self) -> ParserResult<LocalVariableAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableAst::SingleIdentifier, Parser::parse_local_variable_single_identifier));
        Ok(p1)
    }

    fn parse_local_variable_destructure_skip_1_argument(&mut self) -> ParserResult<LocalVariableDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_underscore);
        Ok(LocalVariableDestructureSkip1ArgumentAst::new(c1, p1))
    }

    fn parse_local_variable_destructure_skip_n_arguments(&mut self) -> ParserResult<LocalVariableDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_optional!(self, Parser::parse_local_variable_single_identifier);
        Ok(LocalVariableDestructureSkipNArgumentsAst::new(c1, p1, p2))
    }

    fn parse_local_variable_single_identifier(&mut self) -> ParserResult<LocalVariableSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_optional!(self, Parser::parse_local_variable_single_identifier_alias);
        Ok(LocalVariableSingleIdentifierAst::new(c1, p1, p2, p3))
    }

    fn parse_local_variable_single_identifier_alias(&mut self) -> ParserResult<LocalVariableSingleIdentifierAliasAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_as);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(LocalVariableSingleIdentifierAliasAst::new(c1, p1, p2))
    }

    fn parse_local_variable_destructure_array(&mut self) -> ParserResult<LocalVariableDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_curly_brace);
        let p2 = parse_one_or_more!(self, Parser::parse_local_variable_nested_for_destructure_array, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_curly_brace);
        Ok(LocalVariableDestructureArrayAst::new(c1, p1, p2, p3))
    }

    fn parse_local_variable_destructure_tuple(&mut self) -> ParserResult<LocalVariableDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_one_or_more!(self, Parser::parse_local_variable_nested_for_destructure_tuple, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LocalVariableDestructureTupleAst::new(c1, p1, p2, p3))
    }

    fn parse_local_variable_destructure_object(&mut self) -> ParserResult<LocalVariableDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_single);
        let p2 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p3 = parse_zero_or_more!(self, Parser::parse_local_variable_nested_for_destructure_object, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LocalVariableDestructureObjectAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_local_variable_attribute_binding(&mut self) -> ParserResult<LocalVariableAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_local_variable_nested_for_attribute_binding);
        Ok(LocalVariableAttributeBindingAst::new(c1, p1, p2, p3))
    }

    fn parse_local_variable_nested_for_destructure_array(&mut self) -> ParserResult<LocalVariableNestedForDestructureArrayAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureArrayAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForDestructureArrayAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForDestructureArrayAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForDestructureArrayAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureArrayAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments),
            (LocalVariableNestedForDestructureArrayAst::Skip1Args, Parser::parse_local_variable_destructure_skip_1_argument));
        Ok(p1)
    }

    fn parse_local_variable_nested_for_destructure_tuple(&mut self) -> ParserResult<LocalVariableNestedForDestructureTupleAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureTupleAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForDestructureTupleAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForDestructureTupleAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForDestructureTupleAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureTupleAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments),
            (LocalVariableNestedForDestructureTupleAst::Skip1Args, Parser::parse_local_variable_destructure_skip_1_argument));
        Ok(p1)
    }

    fn parse_local_variable_nested_for_destructure_object(&mut self) -> ParserResult<LocalVariableNestedForDestructureObjectAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForDestructureObjectAst::AttrBind, Parser::parse_local_variable_attribute_binding),
            (LocalVariableNestedForDestructureObjectAst::SingleIdentifier, Parser::parse_local_variable_single_identifier),
            (LocalVariableNestedForDestructureObjectAst::SkipNArgs, Parser::parse_local_variable_destructure_skip_n_arguments));
        Ok(p1)
    }

    fn parse_local_variable_nested_for_attribute_binding(&mut self) -> ParserResult<LocalVariableNestedForAttributeBindingAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (LocalVariableNestedForAttributeBindingAst::DestructureArray, Parser::parse_local_variable_destructure_array),
            (LocalVariableNestedForAttributeBindingAst::DestructureTuple, Parser::parse_local_variable_destructure_tuple),
            (LocalVariableNestedForAttributeBindingAst::DestructureObject, Parser::parse_local_variable_destructure_object),
            (LocalVariableNestedForAttributeBindingAst::SingleIdentifier, Parser::parse_local_variable_single_identifier));
        Ok(p1)
    }

    fn parse_assignment_statement(&mut self) -> ParserResult<AssignmentStatementAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        Ok(AssignmentStatementAst::new(c1, p1, p2, p3))
    }

    fn parse_case_expression_branch_simple(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_pattern_statement_flavour_else_case,
            Parser::parse_pattern_statement_flavour_else);
        Ok(p1)
    }

    fn parse_case_expression_branch(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_pattern_statement_flavour_destructuring,
            Parser::parse_pattern_statement_flavour_non_destructuring,
            Parser::parse_pattern_statement_flavour_else_case,
            Parser::parse_pattern_statement_flavour_else);
        Ok(p1)
    }

    fn parse_pattern_statement_flavour_destructuring(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_is);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_group_destructure, Parser::parse_token_comma);
        let p3 = parse_optional!(self, Parser::parse_pattern_guard);
        let p4 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, p3, p4))
    }

    fn parse_pattern_statement_flavour_non_destructuring(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_boolean_comparison_op);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, Some(p1), p2, None, p3))
    }

    fn parse_pattern_statement_flavour_else_case(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_pattern_variant_else_case);
        Ok(CaseExpressionBranchAst::new_from_else_to_else_case(c1, p1))
    }

    fn parse_pattern_statement_flavour_else(&mut self) -> ParserResult<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_pattern_variant_else);
        let p2 = parse_once!(self, Parser::parse_inner_scope);
        Ok(CaseExpressionBranchAst::new(c1, None, vec![p1], None, p2))
    }

    fn parse_pattern_group_destructure(&mut self) -> ParserResult<PatternVariantAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantAst::DestructureObject, Parser::parse_pattern_variant_destructure_object));
        Ok(p1)
    }

    fn parse_pattern_variant_skip_argument(&mut self) -> ParserResult<PatternVariantDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_underscore);
        Ok(PatternVariantDestructureSkip1ArgumentAst::new(c1, p1))
    }

    fn parse_pattern_variant_skip_arguments(&mut self) -> ParserResult<PatternVariantDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_dot);
        let p2 = parse_optional!(self, Parser::parse_pattern_variant_single_identifier);
        Ok(PatternVariantDestructureSkipNArgumentsAst::new(c1, p1, p2))
    }

    fn parse_pattern_variant_single_identifier(&mut self) -> ParserResult<PatternVariantSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_keyword_mut);
        let p2 = parse_once!(self, Parser::parse_identifier);
        let p3 = parse_optional!(self, Parser::parse_local_variable_single_identifier_alias);
        Ok(PatternVariantSingleIdentifierAst::new(c1, p1, p2, p3))
    }

    fn parse_pattern_variant_destructure_array(&mut self) -> ParserResult<PatternVariantDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_array, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(PatternVariantDestructureArrayAst::new(c1, p1, p2, p3))
    }

    fn parse_pattern_variant_destructure_tuple(&mut self) -> ParserResult<PatternVariantDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_one_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_tuple, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(PatternVariantDestructureTupleAst::new(c1, p1, p2, p3))
    }

    fn parse_pattern_variant_destructure_object(&mut self) -> ParserResult<PatternVariantDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_single);
        let p2 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p3 = parse_zero_or_more!(self, Parser::parse_pattern_variant_nested_for_destructure_object, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(PatternVariantDestructureObjectAst::new(c1, p1, p2, p3, p4))
    }

    fn parse_pattern_variant_attribute_binding(&mut self) -> ParserResult<PatternVariantAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_pattern_variant_nested_for_attribute_binding);
        Ok(PatternVariantAttributeBindingAst::new(c1, p1, p2, p3))
    }

    fn parse_pattern_variant_literal(&mut self) -> ParserResult<PatternVariantLiteralAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantLiteralAst::Float, Parser::parse_literal_float),
            (PatternVariantLiteralAst::Integer, Parser::parse_literal_integer),
            (PatternVariantLiteralAst::String, Parser::parse_literal_string),
            (PatternVariantLiteralAst::Boolean, Parser::parse_literal_boolean));
        Ok(p1)
    }

    fn parse_pattern_variant_expression(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_expression);
        Ok(PatternVariantAst::Expression(PatternVariantExpressionAst::new(c1, p1)))
    }

    fn parse_pattern_variant_else(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        Ok(PatternVariantAst::Else(PatternVariantElseAst::new(c1, p1)))
    }

    fn parse_pattern_variant_else_case(&mut self) -> ParserResult<PatternVariantAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_else);
        let p2 = parse_once!(self, Parser::parse_case_expression);
        Ok(PatternVariantAst::ElseCase(PatternVariantElseCaseAst::new(c1, p1, p2)))
    }

    fn parse_pattern_variant_nested_for_destructure_array(&mut self) -> ParserResult<PatternVariantNestedForDestructureArrayAst> {
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

    fn parse_pattern_variant_nested_for_destructure_tuple(&mut self) -> ParserResult<PatternVariantNestedForDestructureTupleAst> {
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

    fn parse_pattern_variant_nested_for_destructure_object(&mut self) -> ParserResult<PatternVariantNestedForDestructureObjectAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForDestructureObjectAst::AttrBind, Parser::parse_pattern_variant_attribute_binding),
            (PatternVariantNestedForDestructureObjectAst::SingleIdentifier, Parser::parse_pattern_variant_single_identifier),
            (PatternVariantNestedForDestructureObjectAst::SkipNArgs, Parser::parse_pattern_variant_skip_arguments));
        Ok(p1)
    }

    fn parse_pattern_variant_nested_for_attribute_binding(&mut self) -> ParserResult<PatternVariantNestedForAttributeBindingAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PatternVariantNestedForAttributeBindingAst::DestructureArray, Parser::parse_pattern_variant_destructure_array),
            (PatternVariantNestedForAttributeBindingAst::DestructureTuple, Parser::parse_pattern_variant_destructure_tuple),
            (PatternVariantNestedForAttributeBindingAst::DestructureObject, Parser::parse_pattern_variant_destructure_object),
            (PatternVariantNestedForAttributeBindingAst::Literal, Parser::parse_pattern_variant_literal));
        Ok(p1)
    }

    fn parse_pattern_guard(&mut self) -> ParserResult<PatternGuardAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_and);
        let p2 = parse_once!(self, Parser::parse_expression);
        Ok(PatternGuardAst::new(c1, p1, p2))
    }

    fn parse_binary_op_precedence_level_1(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_or);
        Ok(p1)
    }

    fn parse_binary_op_precedence_level_2(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_and);
        Ok(p1)
    }

    fn parse_binary_op_precedence_level_3(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_keyword_is);
        Ok(p1)
    }

    fn parse_binary_op_precedence_level_4(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_eq,
            Parser::parse_token_ne,
            Parser::parse_token_le,
            Parser::parse_token_ge,
            Parser::parse_token_lt,
            Parser::parse_token_gt,
            Parser::parse_token_ss);
        Ok(p1)
    }

    fn parse_binary_op_precedence_level_5(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_add_assign,
            Parser::parse_token_sub_assign,
            Parser::parse_token_add,
            Parser::parse_token_sub);
        Ok(p1)
    }

    fn parse_binary_op_precedence_level_6(&mut self) -> ParserResult<TokenAst> {
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

    fn parse_boolean_comparison_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_eq,
            Parser::parse_token_ne,
            Parser::parse_token_le,
            Parser::parse_token_ge,
            Parser::parse_token_lt,
            Parser::parse_token_gt);
        Ok(p1)
    }

    fn parse_unary_op(&mut self) -> ParserResult<UnaryExpressionOperatorAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (UnaryExpressionOperatorAst::Async, Parser::parse_unary_op_async_call));
        Ok(p1)
    }

    fn parse_unary_op_async_call(&mut self) -> ParserResult<UnaryExpressionOperatorAsyncAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_keyword_async);
        Ok(UnaryExpressionOperatorAsyncAst::new(c1, p1))
    }

    fn parse_postfix_op(&mut self) -> ParserResult<PostfixExpressionOperatorAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (PostfixExpressionOperatorAst::FunctionCall, Parser::parse_postfix_op_function_call),
            (PostfixExpressionOperatorAst::MemberAccess, Parser::parse_postfix_op_member_access),
            (PostfixExpressionOperatorAst::EarlyReturn, Parser::parse_postfix_op_early_return),
            (PostfixExpressionOperatorAst::NotKeyword, Parser::parse_postfix_op_not_keyword),
            (PostfixExpressionOperatorAst::StepKeyword, Parser::parse_postfix_op_step_keyword));
        Ok(p1)
    }

    fn parse_postfix_op_function_call(&mut self) -> ParserResult<PostfixExpressionOperatorFunctionCallAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_generic_arguments);
        let p2 = parse_once!(self, Parser::parse_function_call_arguments);
        let p3 = parse_optional!(self, Parser::parse_token_double_dot);
        Ok(PostfixExpressionOperatorFunctionCallAst::new(c1, p1, p2, p3))
    }

    fn parse_postfix_op_member_access(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_postfix_op_member_access_runtime,
            Parser::parse_postfix_op_member_access_static);
        Ok(p1)
    }

    fn parse_postfix_op_member_access_runtime(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let p2 = parse_alternate!(self,
            Parser::parse_identifier,
            Parser::parse_numeric_identifier);
        Ok(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2))
    }

    fn parse_postfix_op_member_access_static(&mut self) -> ParserResult<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_double_colon);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(PostfixExpressionOperatorMemberAccessAst::new(c1, p1, p2))
    }

    fn parse_postfix_op_early_return(&mut self) -> ParserResult<PostfixExpressionOperatorEarlyReturnAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_question_mark);
        Ok(PostfixExpressionOperatorEarlyReturnAst::new(c1, p1))
    }

    fn parse_postfix_op_not_keyword(&mut self) -> ParserResult<PostfixExpressionOperatorNotKeywordAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let p2 = parse_once!(self, Parser::parse_keyword_not);
        Ok(PostfixExpressionOperatorNotKeywordAst::new(c1, p1, p2))
    }

    fn parse_postfix_op_step_keyword(&mut self) -> ParserResult<PostfixExpressionOperatorStepKeywordAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let p2 = parse_once!(self, Parser::parse_keyword_step);
        Ok(PostfixExpressionOperatorStepKeywordAst::new(c1, p1, p2))
    }

    fn parse_convention(&mut self) -> ParserResult<ConventionAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_convention_mut,
            Parser::parse_convention_ref,
            Parser::parse_convention_mov);
        Ok(p1)
    }

    fn parse_convention_mov(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        Ok(ConventionAst::new_mov(c1))
    }

    fn parse_convention_mut(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_borrow);
        let p2 = parse_once!(self, Parser::parse_keyword_mut);
        Ok(ConventionAst::new_mut(c1, p1, p2))
    }

    fn parse_convention_ref(&mut self) -> ParserResult<ConventionAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_borrow);
        Ok(ConventionAst::new_ref(c1, p1))
    }

    fn parse_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_single);
        let p2 = parse_once!(self, Parser::parse_object_initializer_arguments);
        Ok(ObjectInitializerAst::new(c1, p1, p2))
    }

    fn parse_object_initializer_arguments(&mut self) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_object_initializer_argument, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_object_initializer_argument(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_object_initializer_argument_named,
            Parser::parse_object_initializer_argument_unnamed);
        Ok(p1)
    }

    fn parse_object_initializer_argument_unnamed(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_token_double_dot);
        let p2 = parse_once!(self, Parser::parse_identifier);
        Ok(ObjectInitializerArgumentAst::new_unnamed(c1, p1, p2))
    }

    fn parse_object_initializer_argument_named(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_expression);
        Ok(ObjectInitializerArgumentAst::new_named(c1, p1, p2, Box::new(p3)))
    }

    fn parse_type(&mut self) -> ParserResult<TypeAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_type_optional,
            Parser::parse_type_variant,
            Parser::parse_type_tuple,
            Parser::parse_type_single);
        Ok(p1)
    }

    fn parse_type_optional(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_question_mark);
        let p2 = parse_once!(self, Parser::parse_type);
        Ok(TypeOptionalAst::new(c1, p1, p2).to_type())
    }

    fn parse_type_single(&mut self) -> ParserResult<TypeAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_type_single_with_namespace,
            Parser::parse_type_single_with_self,
            Parser::parse_type_single_without_namespace_or_self);
        Ok(p1)
    }

    fn parse_type_single_with_namespace(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_identifier, Parser::parse_token_double_colon);
        let _a = parse_once!(self, Parser::parse_token_double_colon);
        let p2 = parse_one_or_more!(self, Parser::parse_generic_identifier, Parser::parse_token_double_colon);
        Ok(TypeAst::new(c1, p1, p2))
    }

    fn parse_type_single_with_self(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_self_type);
        let p2 = parse_optional!(self, Parser::parse_types_after_self).unwrap_or_default();
        Ok(TypeAst::new(c1, vec![], vec![p1].into_iter().chain(p2.into_iter()).collect()))
    }

    fn parse_types_after_self(&mut self) -> ParserResult<Vec<GenericIdentifierAst>> {
        let _a = parse_once!(self, Parser::parse_token_double_colon);
        let p1 = parse_zero_or_more!(self, Parser::parse_generic_identifier, Parser::parse_token_double_colon);
        Ok(p1)
    }

    fn parse_type_single_without_namespace_or_self(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_one_or_more!(self, Parser::parse_generic_identifier, Parser::parse_token_double_colon);
        Ok(TypeAst::new(c1, vec![], p1))
    }

    fn parse_self_type(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let _a = parse_once!(self, Parser::parse_keyword_self_type);
        Ok(GenericIdentifierAst::new(c1, "Self".to_string(), None))
    }

    fn parse_type_tuple(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_type, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(TypeTupleAst::new(c1, p1, p2, p3).to_type())
    }

    fn parse_type_non_union(&mut self) -> ParserResult<TypeAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_type_single,
            Parser::parse_type_tuple,
            Parser::parse_type_optional);
        Ok(p1)
    }

    fn parse_type_variant(&mut self) -> ParserResult<TypeAst> {
        let c1 = self.current_pos();
        let p1 = parse_two_or_more!(self, Parser::parse_type_non_union, Parser::parse_token_union);
        Ok(TypeVariantAst::new(c1, p1).to_type())
    }

    fn parse_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_identifier);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    fn parse_numeric_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    fn parse_upper_identifier(&mut self) -> ParserResult<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_upper_identifier);
        Ok(IdentifierAst::new(c1, p1.metadata))
    }

    fn parse_generic_identifier(&mut self) -> ParserResult<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_upper_identifier);
        let p2 = parse_optional!(self, Parser::parse_generic_arguments);
        Ok(GenericIdentifierAst::new(c1, p1.value, p2))
    }

    fn parse_literal(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_float,
            Parser::parse_literal_integer,
            Parser::parse_literal_string,
            Parser::parse_literal_tuple,
            Parser::parse_literal_array,
            Parser::parse_literal_boolean);
        Ok(p1)
    }

    fn parse_literal_float(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_once!(self, Parser::parse_literal_float_b10);
        Ok(p1)
    }

    fn parse_literal_integer(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_integer_b10,
            Parser::parse_literal_integer_b02,
            Parser::parse_literal_integer_b16);
        Ok(p1)
    }

    fn parse_literal_string(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_lexeme_string);
        Ok(LiteralAst::new_string(c1, p1))
    }

    fn parse_literal_tuple(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_tuple_0_items,
            Parser::parse_literal_tuple_1_items,
            Parser::parse_literal_tuple_n_items);
        Ok(p1)
    }

    fn parse_literal_comp_tuple(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_tuple_0_items,
            Parser::parse_literal_comp_tuple_1_items,
            Parser::parse_literal_comp_tuple_n_items);
        Ok(p1)
    }

    fn parse_literal_array(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_array_0_items,
            Parser::parse_literal_array_n_items);
        Ok(p1)
    }

    fn parse_literal_comp_array(&mut self) -> ParserResult<LiteralAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_literal_array_0_items,
            Parser::parse_literal_comp_array_n_items);
        Ok(p1)
    }

    fn parse_literal_boolean(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_alternate!(self,
            Parser::parse_keyword_true,
            Parser::parse_keyword_false);
         Ok(LiteralAst::new_boolean(c1, p1))
    }

    fn parse_literal_float_b10(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p3 = parse_once!(self, Parser::parse_token_dot);
        let p4 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p5 = parse_optional!(self, Parser::parse_float_postfix_type);
        Ok(LiteralAst::new_float(c1, p1, p2, p3, p4, p5.and_then(|x| Some(TypeAst::from(x)))))
    }

    fn parse_literal_integer_b10(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    fn parse_literal_integer_b02(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_bin_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    fn parse_literal_integer_b16(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_optional!(self, Parser::parse_numeric_prefix_op);
        let p2 = parse_once!(self, Parser::parse_lexeme_hex_integer);
        let p3 = parse_optional!(self, Parser::parse_integer_postfix_type);
        Ok(LiteralAst::new_integer(c1, p1, p2, p3.and_then(|x| Some(TypeAst::from(x)))))
    }

    fn parse_numeric_prefix_op(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_alternate!(self,
            Parser::parse_token_sub,
            Parser::parse_token_add);
        Ok(p1)
    }

    fn parse_integer_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
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
            |x| Parser::parse_characters(x, "u256"));
        Ok(IdentifierAst::new(p1.pos, p1.metadata))
    }

    fn parse_float_postfix_type(&mut self) -> ParserResult<IdentifierAst> {
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

    fn parse_literal_tuple_0_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralAst::new_tuple(c1, p1, vec![], p2))
    }

    fn parse_literal_tuple_1_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_expression);
        let _a = parse_once!(self, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralAst::new_tuple(c1, p1, vec![p2], p3))
    }

    fn parse_literal_comp_tuple_1_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_once!(self, Parser::parse_comp_value);
        let _a = parse_once!(self, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralAst::new_tuple(c1, p1, vec![p2], p3))
    }

    fn parse_literal_tuple_n_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_two_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralAst::new_tuple(c1, p1, p2, p3))
    }

    fn parse_literal_comp_tuple_n_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_two_or_more!(self, Parser::parse_comp_value, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(LiteralAst::new_tuple(c1, p1, p2, p3))
    }

    fn parse_literal_array_0_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_once!(self, Parser::parse_type);
        let p3 = parse_once!(self, Parser::parse_token_comma);
        let p4 = parse_once!(self, Parser::parse_lexeme_dec_integer);
        let p5 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralAst::new_array_0(c1, p1, p2, p3, p4, p5))
    }

    fn parse_literal_array_n_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_expression, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralAst::new_array_n(c1, p1, p2, p3))
    }

    fn parse_literal_comp_array_n_items(&mut self) -> ParserResult<LiteralAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_square_bracket);
        let p2 = parse_one_or_more!(self, Parser::parse_comp_value, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_square_bracket);
        Ok(LiteralAst::new_array_n(c1, p1, p2, p3))
    }

    fn parse_comp_value(&mut self) -> ParserResult<ExpressionAst> {
        let p1 = parse_alternate_with_wrapping!(self,
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_float),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_integer),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_string),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_comp_tuple),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_comp_array),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Literal(x)), Parser::parse_literal_boolean),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::Identifier(x)), Parser::parse_identifier),
            (|x| ExpressionAst::Primary(PrimaryExpressionAst::ObjectInitializer(x)), Parser::parse_comp_object_initializer));
        Ok(p1)
    }

    fn parse_comp_object_initializer(&mut self) -> ParserResult<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_type_single);
        let p2 = parse_once!(self, Parser::parse_comp_object_initializer_arguments);
        Ok(ObjectInitializerAst::new(c1, p1, p2))
    }

    fn parse_comp_object_initializer_arguments(&mut self) -> ParserResult<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_token_left_parenthesis);
        let p2 = parse_zero_or_more!(self, Parser::parse_comp_object_initializer_argument_named, Parser::parse_token_comma);
        let p3 = parse_once!(self, Parser::parse_token_right_parenthesis);
        Ok(ObjectInitializerArgumentGroupAst::new(c1, p1, p2, p3))
    }

    fn parse_comp_object_initializer_argument_named(&mut self) -> ParserResult<ObjectInitializerArgumentAst> {
        let c1 = self.current_pos();
        let p1 = parse_once!(self, Parser::parse_identifier);
        let p2 = parse_once!(self, Parser::parse_token_assign);
        let p3 = parse_once!(self, Parser::parse_comp_value);
        Ok(ObjectInitializerArgumentAst::new_named(c1, p1, p2, Box::new(p3)))
    }

    fn parse_keyword(&mut self, keyword: Keywords) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        for c in keyword.to_string().chars() {
            self.parse_character(c)?;
        }
        Ok(TokenAst::new(c1, TokenType::NoToken, keyword.to_string()))
    }

    fn parse_token_left_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftCurlyBrace));
        Ok(p1)
    }

    fn parse_token_right_curly_brace(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightCurlyBrace));
        Ok(p1)
    }

    fn parse_token_left_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftParenthesis));
        Ok(p1)
    }

    fn parse_token_right_parenthesis(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightParenthesis));
        Ok(p1)
    }

    fn parse_token_left_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLeftSquareBracket));
        Ok(p1)
    }

    fn parse_token_right_square_bracket(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkRightSquareBracket));
        Ok(p1)
    }

    fn parse_token_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkDot));
        Ok(p1)
    }

    fn parse_token_comma(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkComma));
        Ok(p1)
    }

    fn parse_token_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkColon));
        Ok(p1)
    }

    fn parse_token_at(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAt));
        Ok(p1)
    }

    fn parse_token_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_newline(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNewLine));
        Ok(p1)
    }

    fn parse_token_question_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkQuestionMark));
        Ok(p1)
    }

    fn parse_token_borrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAmpersand));
        Ok(p1)
    }

    fn parse_token_union(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkVerticalBar));
        Ok(p1)
    }

    fn parse_token_lt(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLessThanSign));
        Ok(p1)
    }

    fn parse_token_gt(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign));
        Ok(p1)
    }

    fn parse_token_add(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPlusSign));
        Ok(p1)
    }

    fn parse_token_sub(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign));
        Ok(p1)
    }

    fn parse_token_mul(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        Ok(p1)
    }

    fn parse_token_div(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkForwardSlash));
        Ok(p1)
    }

    fn parse_token_rem(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        Ok(p1)
    }

    fn parse_token_underscore(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore));
        Ok(p1)
    }

    fn parse_token_speech_mark(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkSpeechMark));
        Ok(p1)
    }

    fn parse_token_double_colon(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_token_colon);
        let _a = parse_once!(self, Parser::parse_token_colon);
        Ok(p1)
    }

    fn parse_token_double_dot(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, Parser::parse_token_dot);
        let _a = parse_once!(self, Parser::parse_token_dot);
        Ok(p1)
    }

    fn parse_token_rightward_arrow(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign));
        Ok(p1)
    }

    fn parse_token_eq(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_ne(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkExclamationMark));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_le(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLessThanSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_ge(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_add_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPlusSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_sub_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkMinusSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_ss(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkLessThanSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkGreaterThanSign));
        Ok(p1)
    }

    fn parse_token_mul_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_div_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkForwardSlash));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_rem_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_mod_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_exp_assign(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkEqualsSign));
        Ok(p1)
    }

    fn parse_token_mod(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkPercentSign));
        Ok(p1)
    }

    fn parse_token_exp(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        let _a = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkAsterisk));
        Ok(p1)
    }

    fn parse_token_no_token(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::NoToken));
        Ok(p1)
    }

    fn parse_keyword_cls(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cls));
        Ok(p1)
    }

    fn parse_keyword_sup(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Sup));
        Ok(p1)
    }

    fn parse_keyword_ext(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Ext));
        Ok(p1)
    }

    fn parse_keyword_fun(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Fun));
        Ok(p1)
    }

    fn parse_keyword_cor(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cor));
        Ok(p1)
    }

    fn parse_keyword_use(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Use));
        Ok(p1)
    }

    fn parse_keyword_let(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Let));
        Ok(p1)
    }

    fn parse_keyword_mut(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Mut));
        Ok(p1)
    }

    fn parse_keyword_cmp(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Cmp));
        Ok(p1)
    }

    fn parse_keyword_where(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Where));
        Ok(p1)
    }

    fn parse_keyword_self_val(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::SelfVal));
        Ok(p1)
    }

    fn parse_keyword_self_type(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::SelfType));
        Ok(p1)
    }

    fn parse_keyword_case(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Case));
        Ok(p1)
    }

    fn parse_keyword_loop(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Loop));
        Ok(p1)
    }

    fn parse_keyword_with(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::With));
        Ok(p1)
    }

    fn parse_keyword_gen(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Gen));
        Ok(p1)
    }

    fn parse_keyword_ret(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Ret));
        Ok(p1)
    }

    fn parse_keyword_skip(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Skip));
        Ok(p1)
    }

    fn parse_keyword_exit(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Exit));
        Ok(p1)
    }

    fn parse_keyword_else(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Else));
        Ok(p1)
    }

    fn parse_keyword_false(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::False));
        Ok(p1)
    }

    fn parse_keyword_true(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::True));
        Ok(p1)
    }

    fn parse_keyword_of(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Of));
        Ok(p1)
    }

    fn parse_keyword_in(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::In));
        Ok(p1)
    }

    fn parse_keyword_pin(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Pin));
        Ok(p1)
    }

    fn parse_keyword_rel(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Rel));
        Ok(p1)
    }

    fn parse_keyword_as(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::As));
        Ok(p1)
    }

    fn parse_keyword_is(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Is));
        Ok(p1)
    }

    fn parse_keyword_and(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::And));
        Ok(p1)
    }

    fn parse_keyword_or(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Or));
        Ok(p1)
    }

    fn parse_keyword_not(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Not));
        Ok(p1)
    }

    fn parse_keyword_async(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Async));
        Ok(p1)
    }

    fn parse_keyword_step(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_keyword(x, Keywords::Step));
        Ok(p1)
    }

    fn parse_lexeme_dec_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();

        parse_once!(self, Parser::parse_token_no_token);
        match self.tokens[self.current_pos()] {
            TokenType::TkNumber(num) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num)));
                integer.push(num);
            }
            _ => return Err(SyntaxError::new(c1, "Expected decimal integer".to_string())),
        }

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num)));
            integer.push(p1.metadata.as_bytes()[0] as char);
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected decimal integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenType::NoToken, integer))
        }
    }

    fn parse_lexeme_bin_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber('0')));
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter('b')));

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num)));
            integer.push(p1.metadata.as_bytes()[0] as char);
            if !num.is_digit(2) {
                return Err(SyntaxError::new(c1, "Expected binary integer".to_string()));
            }
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected binary integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenType::NoToken, integer))
        }
    }

    fn parse_lexeme_hex_integer(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut integer = "".to_string();
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber('0')));
        parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter('x')));

        while let TokenType::TkNumber(num) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(num)));
            integer.push(p1.metadata.as_bytes()[0] as char);
            if !num.is_digit(16) {
                return Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()));
            }
        }

        if integer.is_empty() {
            Err(SyntaxError::new(c1, "Expected hexadecimal integer".to_string()))
        } else {
            Ok(TokenAst::new(c1, TokenType::NoToken, integer))
        }
    }

    fn parse_lexeme_string(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();
        parse_once!(self, Parser::parse_token_speech_mark);

        while let TokenType::TkCharacter(string) = self.tokens[self.current_pos()] {
            let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }

        parse_once!(self, Parser::parse_token_speech_mark);
        Ok(TokenAst::new(c1, TokenType::NoToken, identifier))
    }

    fn parse_lexeme_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        parse_once!(self, Parser::parse_token_no_token);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) if string.is_lowercase() => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
                identifier.push(string);
            },
            _ => return Err(SyntaxError::new(c1, "Expected identifier".to_string())),
        }

        loop {
            match self.tokens[self.current_pos()] {
                TokenType::TkCharacter(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
                    identifier.push(string);
                },
                TokenType::TkNumber(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string)));
                    identifier.push(string);
                },
                TokenType::TkUnderscore => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore));
                    identifier.push('_');
                },
                _ => break,
            }
        }

        if identifier.is_empty() {
            return Err(SyntaxError::new(c1, "Expected identifier".to_string()));
        }

        Ok(TokenAst::new(c1, TokenType::NoToken, identifier))
    }

    fn parse_lexeme_upper_identifier(&mut self) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        parse_once!(self, Parser::parse_token_no_token);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) if string.is_uppercase() => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
                identifier.push(string);
            },
            _ => return Err(SyntaxError::new(c1, "Expected upper identifier".to_string())),
        }

        loop {
            match self.tokens[self.current_pos()] {
                TokenType::TkCharacter(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
                    identifier.push(string);
                },
                TokenType::TkNumber(string) => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string)));
                    identifier.push(string);
                },
                TokenType::TkUnderscore => {
                    parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkUnderscore));
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

        Ok(TokenAst::new(c1, TokenType::NoToken, identifier))
    }

    fn parse_characters(&mut self, characters: &str) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();
        let mut identifier = "".to_string();

        for c in characters.chars() {
            let p1 = parse_once!(self, |x| Parser::parse_character(x, c));
            identifier.push(p1.metadata.as_bytes()[0] as char);
        }

        if identifier != characters {
            return Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", characters, identifier)));
        }

        Ok(TokenAst::new(c1, TokenType::NoToken, "".to_string()))
    }

    fn parse_character(&mut self, character: char) -> ParserResult<TokenAst> {
        let c1 = self.current_pos();

        parse_once!(self, Parser::parse_token_no_token);
        match self.tokens[self.current_pos()] {
            TokenType::TkCharacter(string) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkCharacter(string)));
                if string != character {
                    Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", character, string)))
                }
                else {
                    Ok(TokenAst::new(c1, TokenType::NoToken, string.to_string()))
                }
            },
            TokenType::TkNumber(string) => {
                parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::TkNumber(string)));
                if string != character {
                    Err(SyntaxError::new(c1, format!("Expected '{}', got '{}'", character, string)))
                }
                else {
                    Ok(TokenAst::new(c1, TokenType::NoToken, string.to_string()))
                }
            },
            _ => Err(SyntaxError::new(c1, "Expected upper identifier".to_string())),
        }
    }

    fn parse_eof(&mut self) -> ParserResult<TokenAst> {
        let p1 = parse_once!(self, |x| Parser::parse_token_primitive(x, TokenType::EndOfFile));
        Ok(p1)
    }

    fn parse_token_primitive(&mut self, token_type: TokenType) -> ParserResult<TokenAst> {
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
            return Ok(TokenAst::new(self.current_pos(), TokenType::NoToken, "".to_string()));
        }

        if self.tokens[self.index] != token_type {
            if self.error.get_pos() == self.index {
                self.error.add_expected_token(token_type);
                return Err(self.error.clone());
            }

            let new_error = format!("Expected got '{}'", self.tokens[self.index]);
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

        let r = TokenAst::new(self.current_pos(), self.tokens[self.index].clone(), token_character);
        self.index += 1;
        Ok(r)
    }
}
