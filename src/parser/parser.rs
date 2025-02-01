use SPP_Compiler_Rust::parser_rule;
use crate::asts::annotation_ast::AnnotationAst;
use crate::asts::binary_expression_ast::BinaryExpressionAst;
use crate::asts::case_expression_ast::CaseExpressionAst;
use crate::asts::class_attribute_ast::ClassAttributeAst;
use crate::asts::class_implementation_ast::ClassImplementationAst;
use crate::asts::class_member_ast::ClassMemberAst;
use crate::asts::class_prototype_ast::ClassPrototypeAst;
use crate::asts::coroutine_prototype_ast::CoroutinePrototypeAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::function_call_argument::FunctionCallArgumentAst;
use crate::asts::function_call_argument_group_ast::FunctionCallArgumentGroupAst;
use crate::asts::function_call_argument_named_ast::FunctionCallArgumentNamedAst;
use crate::asts::function_call_argument_unnamed_ast::FunctionCallArgumentUnnamedAst;
use crate::asts::function_implementation_ast::FunctionImplementationAst;
use crate::asts::function_member_ast::FunctionMemberAst;
use crate::asts::function_parameter_ast::FunctionParameterAst;
use crate::asts::function_parameter_group_ast::FunctionParameterGroupAst;
use crate::asts::function_parameter_optional_ast::FunctionParameterOptionalAst;
use crate::asts::function_parameter_required_ast::FunctionParameterRequiredAst;
use crate::asts::function_parameter_self_ast::FunctionParameterSelfAst;
use crate::asts::function_parameter_variadic_ast::FunctionParameterVariadicAst;
use crate::asts::function_prototype_ast::{FunctionPrototypeAst, FunctionPrototypeBaseAst};
use crate::asts::generic_argument_ast::GenericArgumentAst;
use crate::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::asts::generic_argument_comp_named_ast::GenericArgumentCompNamedAst;
use crate::asts::generic_argument_comp_unnamed_ast::GenericArgumentCompUnnamedAst;
use crate::asts::generic_argument_type_named_ast::GenericArgumentTypeNamedAst;
use crate::asts::generic_argument_type_unnamed_ast::GenericArgumentTypeUnnamedAst;
use crate::asts::generic_parameter_ast::GenericParameterAst;
use crate::asts::generic_parameter_comp_optional_ast::GenericParameterCompOptionalAst;
use crate::asts::generic_parameter_comp_required::GenericParameterCompRequiredAst;
use crate::asts::generic_parameter_comp_variadic_ast::GenericParameterCompVariadicAst;
use crate::asts::generic_parameter_group_ast::GenericParameterGroupAst;
use crate::asts::generic_parameter_type_inline_constraints_ast::GenericParameterTypeInlineConstraintsAst;
use crate::asts::generic_parameter_type_optional::GenericParameterTypeOptionalAst;
use crate::asts::generic_parameter_type_required::GenericParameterTypeRequiredAst;
use crate::asts::generic_parameter_type_variadic::GenericParameterTypeVariadicAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::module_implementation_ast::ModuleImplementationAst;
use crate::asts::module_member_ast::ModuleMemberAst;
use crate::asts::module_prototype_ast::ModulePrototypeAst;
use crate::asts::parenthesized_expression_ast::ParenthesizedExpressionAst;
use crate::asts::postfix_expression_ast::PostfixExpressionAst;
use crate::asts::subroutine_prototype_ast::SubroutinePrototypeAst;
use crate::asts::sup_implementation_ast::SupImplementationAst;
use crate::asts::sup_member_ast::SupMemberAst;
use crate::asts::sup_prototype_extension_ast::SupPrototypeExtensionAst;
use crate::asts::sup_prototype_functions_ast::SupPrototypeFunctionsAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;
use crate::asts::unary_expression_ast::UnaryExpressionAst;
use crate::asts::where_block_ast::WhereBlockAst;
use crate::asts::where_constraints_ast::WhereConstraintsAst;
use crate::asts::where_constraints_group_ast::WhereConstraintsGroupAst;
use crate::lexer::token::{Keywords, TokenStream, TokenType, KEYWORD_STRINGS};
use crate::parser::parser_rule_handler::ParserRuleHandler;


type ParserReturnType<T> = Box<dyn ParserRuleHandler<T>>;


pub struct Parser {
    pub tokens: TokenStream,
    pub index: usize,
}

impl Parser {
    fn current_index(&self) -> usize {
        self.index
    }
}


impl Parser {
    #[parser_rule]
    fn parse_module_prototype(&mut self) -> ParserReturnType<ModulePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_module_implementation().parse_once()?;
        return ModulePrototypeAst::new(c1, p1);
    }

    #[parser_rule]
    fn parse_module_implementation(&mut self) -> ParserReturnType<ModuleImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_module_member().parse_zero_or_more(TokenType::NoToken)?;
        return ModuleImplementationAst::new(c1, p1);
    }

    #[parser_rule]
    fn parse_module_member(&mut self) -> ParserReturnType<ModuleMemberAst> {
        let p1 = self.parse_function_prototype();
        let p2 = self.parse_class_prototype();
        let p3 = self.parse_sup_prototype_extension();
        let p4 = self.parse_sup_prototype_functions();
        let p5 = self.parse_global_use_statement();
        let p6 = self.parse_global_constant();
        let p7 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p7;
    }

    #[parser_rule]
    fn parse_class_prototype(&mut self) -> ParserReturnType<ClassPrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_keyword(Keywords::Cls).parse_once()?;
        let p3 = self.parse_upper_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_where_block().parse_optional();
        let p6 = self.parse_class_implementation().parse_once()?;
        return ClassPrototypeAst::new(c1, p1, p2, TypeAst::from_identifier(p3), p4, p5, p6);
    }

    #[parser_rule]
    fn parse_class_implementation(&mut self) -> ParserReturnType<ClassImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self.parse_class_member().parse_zero_or_more(TokenType::NoToken)?;
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return ClassImplementationAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_class_member(&mut self) -> ParserReturnType<ClassMemberAst> {
        let p1 = self.parse_class_attribute().parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_class_attribute(&mut self) -> ParserReturnType<ClassAttributeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token_colon().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return ClassAttributeAst::new(c1, p1, p2, p3, p4);
    }

    #[parser_rule]
    fn parse_sup_prototype_functions(&mut self) -> ParserReturnType<SupPrototypeFunctionsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Sup).parse_once()?;
        let p2 = self.parse_generic_parameters().parse_optional();
        let p3 = self.parse_type().parse_once()?;
        let p4 = self.parse_where_block().parse_optional();
        let p5 = self.parse_sup_implementation().parse_once()?;
        return SupPrototypeFunctionsAst::new(c1, p1, p2, p3, p4, p5);
    }

    #[parser_rule]
    fn parse_sup_prototype_extension(&mut self) -> ParserReturnType<SupPrototypeExtensionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Sup).parse_once()?;
        let p2 = self.parse_generic_parameters().parse_optional();
        let p3 = self.parse_type().parse_once()?;
        let p4 = self.parse_keyword(Keywords::Ext).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        let p6 = self.parse_where_block().parse_optional();
        let p7 = self.parse_sup_implementation().parse_once()?;
        return SupPrototypeExtensionAst::new(c1, p1, p2, p3, p6, p7, p4, p5);
    }

    #[parser_rule]
    fn parse_sup_implementation(&mut self) -> ParserReturnType<SupImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_curly_brace().parse_once()?;
        let p2 = self.parse_sup_member().parse_zero_or_more(TokenType::NoToken);
        let p3 = self.parse_token_right_curly_brace().parse_once()?;
        return SupImplementationAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_sup_member(&mut self) -> ParserReturnType<SupMemberAst> {
        let p1 = self.parse_sup_method_prototype();
        let p2 = self.parse_sup_use_statement();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }

    #[parser_rule]
    fn parse_sup_method_prototype(&mut self) -> ParserReturnType<FunctionPrototypeAst> {
        let p1 = self.parse_function_prototype().parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_sup_use_statement(&mut self) -> ParserReturnType<SupUseStatementAst> {
        let p1 = self.parse_annotation().parse_zero_or_more(self.parse_token_newline);
        let p2 = self.parse_use_statement().parse_once()?;
        p2.annotations = p1;
        return p2;
    }

    #[parser_rule]
    fn parse_function_prototype(&mut self) -> ParserReturnType<FunctionPrototypeAst> {
        let p1 = self.parse_subroutine_prototype();
        let p2 = self.parse_coroutine_prototype();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }

    #[parser_rule]
    fn parse_subroutine_prototype(&mut self) -> ParserReturnType<SubroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_keyword(Keywords::Fun).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_function_parameters().parse_once()?;
        let p6 = self.parse_token_rightward_arrow().parse_once()?;
        let p7 = self.parse_type().parse_once()?;
        let p8 = self.parse_where_block().parse_optional();
        let p9 = self.parse_function_implementation().parse_once()?;
        return SubroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9));
    }

    #[parser_rule]
    fn parse_coroutine_prototype(&mut self) -> ParserReturnType<CoroutinePrototypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_token(Keywords::Cor).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_generic_parameters().parse_optional();
        let p5 = self.parse_function_parameters().parse_once()?;
        let p6 = self.parse_token_rightward_arrow().parse_once()?;
        let p7 = self.parse_type().parse_once()?;
        let p8 = self.parse_where_block().parse_optional();
        let p9 = self.parse_function_implementation().parse_once()?;
        return CoroutinePrototypeAst(FunctionPrototypeBaseAst::new(c1, p1, p2, p3, p4, p5, p6, p7, p8, p9));
    }

    #[parser_rule]
    fn parse_function_implementation(&mut self) -> ParserReturnType<FunctionImplementationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkLeftCurlyBrace).parse_once()?;
        let p2 = self.parse_function_member().parse_zero_or_more(TokenType::NoToken);
        let p3 = self.parse_token(TokenType::TkRightCurlyBrace).parse_once()?;
        return FunctionImplementationAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_function_member(&mut self) -> ParserReturnType<FunctionMemberAst> {
        let p1 = self.parse_statement().parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_function_call_arguments(&mut self) -> ParserReturnType<FunctionCallArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_function_call_argument().parse_zero_or_more(TokenType::TkComma);
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return FunctionCallArgumentGroupAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_function_call_argument(&mut self) -> ParserReturnType<FunctionCallArgumentAst> {
        let p1 = self.parse_function_call_argument_named();
        let p2 = self.parse_function_call_argument_unnamed();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }

    #[parser_rule]
    fn parse_function_call_argument_unnamed(&mut self) -> ParserReturnType<FunctionCallArgumentUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_convention().parse_once()?;
        let p2 = self.parse_token_double_dot().parse_optional();
        let p3 = self.parse_expression().parse_once()?;
        return FunctionCallArgumentUnnamedAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_function_call_argument_named(&mut self) -> ParserReturnType<FunctionCallArgumentNamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_expression().parse_once()?;
        return FunctionCallArgumentNamedAst::new(c1, p1, p2, p3, p4);
    }

    #[parser_rule]
    fn parse_function_parameters(&mut self) -> ParserReturnType<FunctionParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_function_parameter().parse_zero_or_more(TokenType::TkComma);
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return FunctionParameterGroupAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_function_parameter(&mut self) -> ParserReturnType<FunctionParameterAst> {
        let p1 = self.parse_function_parameter_variadic();
        let p2 = self.parse_function_parameter_optional();
        let p3 = self.parse_function_parameter_required();
        let p4 = self.parse_function_parameter_self();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }

    #[parser_rule]
    fn parse_function_parameter_self(&mut self) -> ParserReturnType<FunctionParameterSelfAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Mut).parse_optional();
        let p2 = self.parse_convention().parse_once()?;
        let p3 = self.parse_self_keyword().parse_once()?;
        return FunctionParameterSelfAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_function_parameter_required(&mut self) -> ParserReturnType<FunctionParameterRequiredAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return FunctionParameterRequiredAst::new(c1, p1, p2, p3, p4);
    }

    #[parser_rule]
    fn parse_function_parameter_optional(&mut self) -> ParserReturnType<FunctionParameterOptionalAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p3 = self.parse_convention().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        let p5 = self.parse_token_assign().parse_once()?;
        let p6 = self.parse_expression().parse_once()?;
        return FunctionParameterOptionalAst::new(c1, p1, p2, p3, p4, p5, p6);
    }

    #[parser_rule]
    fn parse_function_parameter_variadic(&mut self) -> ParserReturnType<FunctionParameterVariadicAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_convention().parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return FunctionParameterVariadicAst::new(c1, p1, p2, p3, p4, p5);
    }

    #[parser_rule]
    fn parse_generic_arguments(&mut self) -> ParserReturnType<GenericArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_generic_argument().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return GenericArgumentGroupAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_generic_argument(&mut self) -> ParserReturnType<GenericArgumentAst> {
        let p1 = self.parse_generic_type_argument_named();
        let p2 = self.parse_generic_type_argument_unnamed();
        let p3 = self.parse_generic_comp_argument_named();
        let p4 = self.parse_generic_comp_argument_unnamed();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }

    #[parser_rule]
    fn parse_generic_type_argument_named(&mut self) -> ParserReturnType<GenericArgumentTypeNamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_type().parse_once()?;
        return GenericArgumentTypeNamedAst::new(c1, TypeAst::from(p1), p2, p3);
    }

    #[parser_rule]
    fn parse_generic_type_argument_unnamed(&mut self) -> ParserReturnType<GenericArgumentTypeUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type().parse_once()?;
        return GenericArgumentTypeUnnamedAst::new(c1, p1);
    }

    #[parser_rule]
    fn parse_generic_comp_argument_named(&mut self) -> ParserReturnType<GenericArgumentCompNamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token_assign().parse_once()?;
        let p3 = self.parse_global_constant_value().parse_once()?;
        return GenericArgumentCompNamedAst::new(c1, TypeAst::from(p1), p2, p3);
    }

    #[parser_rule]
    fn parse_generic_comp_argument_unnamed(&mut self) -> ParserReturnType<GenericArgumentCompUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_global_constant_value().parse_once()?;
        return GenericArgumentCompUnnamedAst::new(c1, p1);
    }

    #[parser_rule]
    fn parse_generic_parameters(&mut self) -> ParserReturnType<GenericParameterGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_generic_parameter().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return GenericParameterGroupAst::new(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_generic_parameter(&mut self) -> ParserReturnType<GenericParameterAst> {
        let p1 = self.parse_generic_comp_parameter_variadic();
        let p2 = self.parse_generic_comp_parameter_optional();
        let p3 = self.parse_generic_comp_parameter_required();
        let p4 = self.parse_generic_type_parameter_variadic();
        let p5 = self.parse_generic_type_parameter_optional();
        let p6 = self.parse_generic_type_parameter_required();
        let p7 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p7;
    }

    #[parser_rule]
    fn parse_generic_type_parameter_required(&mut self) -> ParserReturnType<GenericParameterTypeRequiredAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_inline_constraints().parse_optional();
        return GenericParameterTypeRequiredAst::new(c1, TypeAst::from(p1), p2);
    }

    #[parser_rule]
    fn parse_generic_type_parameter_optional(&mut self) -> ParserReturnType<GenericParameterTypeOptionalAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_inline_constraints().parse_optional();
        let p3 = self.parse_token_assign().parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return GenericParameterTypeOptionalAst::new(c1, TypeAst::from(p1), p2, p3, p4);
    }

    #[parser_rule]
    fn parse_generic_type_parameter_variadic(&mut self) -> ParserReturnType<GenericParameterTypeVariadicAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_double_dot().parse_once()?;
        let p2 = self.parse_upper_identifier().parse_once()?;
        let p3 = self.parse_generic_inline_constraints().parse_optional();
        return GenericParameterTypeVariadicAst::new(c1, p1, TypeAst::from(p2), p3);
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_required(&mut self) -> ParserReturnType<GenericParameterCompRequiredAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return GenericParameterCompRequiredAst::new(c1, p1, TypeAst::from(p2), p3, p4);
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_optional(&mut self) -> ParserReturnType<GenericParameterCompOptionalAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        let p5 = self.parse_token_assign().parse_once()?;
        let p6 = self.parse_global_constant_value().parse_once()?;
        return GenericParameterCompOptionalAst::new(c1, p1, TypeAst::from(p2), p3, p4, p5, p6);
    }

    #[parser_rule]
    fn parse_generic_comp_parameter_variadic(&mut self) -> ParserReturnType<GenericParameterCompVariadicAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Cmp).parse_once()?;
        let p2 = self.parse_token_double_dot().parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return GenericParameterCompVariadicAst::new(c1, p1, p2, TypeAst::from(p3), p4, p5);
    }

    #[parser_rule]
    fn parse_generic_inline_constraints(&mut self) -> ParserReturnType<GenericParameterTypeInlineConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p2 = self.parse_type().parse_one_or_more(TokenType::TkComma);
        return GenericParameterTypeInlineConstraintsAst::new(c1, p1, (p2));
    }

    #[parser_rule]
    fn parse_where_block(&mut self) -> ParserReturnType<WhereBlockAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Where).parse_once()?;
        let p2 = self.parse_where_block_constraints_group().parse_once()?;
        return WhereBlockAst::new(c1, p1, p2);
    }

    #[parser_rule]
    fn parse_where_block_constraints_group(&mut self) -> ParserReturnType<WhereConstraintsGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_square_bracket().parse_once()?;
        let p2 = self.parse_where_block_constraints().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token_right_square_bracket().parse_once()?;
        return WhereConstraintsGroupAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_where_block_constraints(&mut self) -> ParserReturnType<WhereConstraintsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type().parse_one_or_more(TokenType::TkComma);
        let p2 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p3 = self.parse_type().parse_once()?;
        return WhereConstraintsAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_annotation(&mut self) -> ParserReturnType<AnnotationAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkAt).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return AnnotationAst::new(c1, p1, p2);
    }

    #[parser_rule]
    fn parse_expression(&mut self) -> ParserReturnType<ExpressionAst> {
        let p1 = self.parse_binary_expression_precedence_level_1().parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_binary_expression_precedence_level_n_rhs(self, op: fn() -> ParserReturnType<ExpressionAst>, rhs: fn() -> ParserReturnType<TokenAst>) -> ParserReturnType<(TokenAst, ExpressionAst)> {
        let p1 = op().parse_once()?;
        let p2 = rhs().parse_once()?;
        return (p1, p2);
    }

    #[parser_rule]
    fn parse_binary_expression_precedence_level_n(self, lhs: fn() -> ParserReturnType<ExpressionAst>, op: fn() -> ParserReturnType<TokenAst>, rhs: fn() -> ParserReturnType<ExpressionAst>) -> ParserReturnType<ExpressionAst> {
        let c1 = self.current_index();
        let p1 = lhs().parse_once()?;
        let p2 = self.parse_binary_expression_precedence_level_n_rhs(op, rhs).parse_optional();
        return if p2 { ExpressionAst::Binary(BinaryExpressionAst::new(c1, Box::from(p1), p2[0], p2[1])) } else { p1 };
    }

    fn parse_binary_expression_precedence_level_1(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_binary_expression_precedence_level_2(),
            || self.parse_binary_op_precedence_level_1(),
            || self.parse_binary_expression_precedence_level_1())
    }

    fn parse_binary_expression_precedence_level_2(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_binary_expression_precedence_level_3(),
            || self.parse_binary_op_precedence_level_2(),
            || self.parse_binary_expression_precedence_level_2())
    }

    fn parse_binary_expression_precedence_level_3(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_binary_expression_precedence_level_4(),
            || self.parse_binary_op_precedence_level_3(),
            || self.parse_pattern_group_destructure())
    }

    fn parse_binary_expression_precedence_level_4(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_binary_expression_precedence_level_5(),
            || self.parse_binary_op_precedence_level_4(),
            || self.parse_binary_expression_precedence_level_4())
    }

    fn parse_binary_expression_precedence_level_5(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_binary_expression_precedence_level_6(),
            || self.parse_binary_op_precedence_level_5(),
            || self.parse_binary_expression_precedence_level_5())
    }

    fn parse_binary_expression_precedence_level_6(&mut self) -> ParserReturnType<ExpressionAst> {
        self.parse_binary_expression_precedence_level_n(
            || self.parse_unary_expression(),
            || self.parse_binary_op_precedence_level_6(),
            || self.parse_binary_expression_precedence_level_6())
    }

    #[parser_rule]
    fn parse_unary_expression(&mut self) -> ParserReturnType<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_unary_op().parse_zero_or_more(TokenType::NoToken);
        let p2 = self.parse_postfix_expression().parse_once()?;
        return p1.into_iter().rev().fold(p2, |acc, x| UnaryExpressionAst::new(c1, x, acc));
    }

    #[parser_rule]
    fn parse_postfix_expression(&mut self) -> ParserReturnType<ExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_primary_expression().parse_once()?;
        let p2 = self.parse_postfix_op().parse_zero_or_more(TokenType::NoToken);
        return p2.into_iter().fold(p1, |acc, x| PostfixExpressionAst::new(c1, acc, x));
    }

    #[parser_rule]
    fn parse_primary_expression(&mut self) -> ParserReturnType<ExpressionAst> {
        let p1 = self.parse_literal();
        let p2 = self.parse_object_initializer();
        let p4 = self.parse_parenthesized_expression();
        let p5 = self.parse_type();
        let p6 = self.parse_identifier();
        let p7 = self.parse_case_expression();
        let p8 = self.parse_loop_expression();
        let p9 = self.parse_gen_expression();
        let p10 = self.parse_with_expression();
        let p11 = self.parse_inner_scope();
        let p12 = self.parse_self_keyword();
        let p13 = self.parse_token_double_dot();
        let p14 = (p1 | p2 | p4 | p5 | p6 | p7 | p8 | p9 | p10 | p11 | p12 | p13).parse_once()?;
        return p14;
    }

    #[parser_rule]
    fn parse_parenthesized_expression(&mut self) -> ParserReturnType<ParenthesizedExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token_left_parenthesis().parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_token_right_parenthesis().parse_once()?;
        return ParenthesizedExpressionAst::new(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_self_keyword(&mut self) -> ParserReturnType<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::SelfVal_).parse_once()?;
        return IdentifierAst::new(c1, p1.metadata);
    }

    #[parser_rule]
    fn parse_case_expression(&mut self) -> ParserReturnType<CaseExpressionAst> {
        let p1 = self.parse_case_expression_patterns();
        let p2 = self.parse_case_expression_simple();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_case_expression_patterns(&mut self) -> ParserReturnType<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_keyword(Keywords::Case).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_keyword(Keywords::Of).parse_once()?;
        let p4 = self.parse_case_expression_branch().parse_one_or_more(TokenType::NoToken);
        return CaseExpressionAst::new(c1, p1, p2, p3, p4);
    }
    #[parser_rule]
    fn parse_case_expression_simple(&mut self) -> ParserReturnType<CaseExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwCase).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        let p3 = self.parse_inner_scope().parse_once()?;
        let p4 = self.parse_case_expression_branch_simple().parse_zero_or_more(TokenType::NoToken);
        return CaseExpressionAst.from_simple(c1, p1, p2, p3, (p4));
    }
    #[parser_rule]
    fn parse_loop_expression(&mut self) -> ParserReturnType<LoopExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwLoop).parse_once()?;
        let p2 = self.parse_loop_expression_condition().parse_once()?;
        let p3 = self.parse_inner_scope().parse_once()?;
        let p4 = self.parse_loop_else_statement().parse_optional();
        return LoopExpressionAst(c1, p1, p2, p3, p4);
    }
    #[parser_rule]
    fn parse_loop_expression_condition(&mut self) -> ParserReturnType<LoopConditionAst> {
        let p1 = self.parse_loop_expression_condition_iterable();
        let p2 = self.parse_loop_expression_condition_boolean();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_loop_expression_condition_boolean(&mut self) -> ParserReturnType<LoopConditionBooleanAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_once()?;
        return LoopConditionBooleanAst(c1, p1);
    }
    #[parser_rule]
    fn parse_loop_expression_condition_iterable(&mut self) -> ParserReturnType<LoopConditionIterableAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token(TokenType::KwIn).parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return LoopConditionIterableAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_loop_else_statement(&mut self) -> ParserReturnType<LoopElseStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwElse).parse_once()?;
        let p2 = self.parse_inner_scope().parse_once()?;
        return LoopElseStatementAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_gen_expression(&mut self) -> ParserReturnType<GenExpressionAst> {
        let p1 = self.parse_gen_expression_unroll();
        let p2 = self.parse_gen_expression_normal();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_gen_expression_normal(&mut self) -> ParserReturnType<GenExpressionAst> {
        let p1 = self.parse_gen_expression_normal_with_expression();
        let p2 = self.parse_gen_expression_normal_no_expression();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_gen_expression_normal_no_expression(&mut self) -> ParserReturnType<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwGen).parse_once()?;
        return GenExpressionAst(c1, p1, None, ConventionMovAst(p1.pos), None);
    }
    #[parser_rule]
    fn parse_gen_expression_normal_with_expression(&mut self) -> ParserReturnType<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwGen).parse_once()?;
        let p2 = self.parse_convention().parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return GenExpressionAst(c1, p1, None, p2, p3);
    }
    #[parser_rule]
    fn parse_gen_expression_unroll(&mut self) -> ParserReturnType<GenExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwGen).parse_once()?;
        let p2 = self.parse_token(TokenType::KwWith).parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return GenExpressionAst(c1, p1, p2, ConventionMovAst(p3.pos), p3);
    }
    #[parser_rule]
    fn parse_with_expression(&mut self) -> ParserReturnType<WithExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwWith).parse_once()?;
        let p2 = self.parse_with_expression_lhs_alias().parse_optional();
        let p3 = self.parse_expression().parse_once()?;
        let p4 = self.parse_inner_scope().parse_once()?;
        return WithExpressionAst(c1, p1, p2, p3, p4);
    }
    #[parser_rule]
    fn parse_with_expression_lhs_alias(&mut self) -> ParserReturnType<WithExpressionAliasAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_local_variable().parse_once()?;
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        return WithExpressionAliasAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_ret_statement(&mut self) -> ParserReturnType<RetStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwRet).parse_once()?;
        let p2 = self.parse_expression().parse_optional();
        return RetStatementAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_exit_statement(&mut self) -> ParserReturnType<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwExit).parse_one_or_more(TokenType::NoToken);
        let p2 = self.parse_exit_statement_final_action().parse_optional();
        return LoopControlFlowStatementAst(c1, (p1), p2);
    }
    #[parser_rule]
    fn parse_exit_statement_final_action(&mut self) -> ParserReturnType<ExitStatementFinalActionAst> {
        let p1 = self.parse_token(TokenType::KwSkip);
        let p2 = self.parse_expression();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_skip_statement(&mut self) -> ParserReturnType<LoopControlFlowStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwSkip).parse_once()?;
        return LoopControlFlowStatementAst(c1, (), p1);
    }
    #[parser_rule]
    fn parse_pin_statement(&mut self) -> ParserReturnType<PinStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwPin).parse_once()?;
        let p2 = self.parse_expression().parse_one_or_more(TokenType::TkComma);
        return PinStatementAst(c1, p1, (p2));
    }
    #[parser_rule]
    fn parse_rel_statement(&mut self) -> ParserReturnType<RelStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwRel).parse_once()?;
        let p2 = self.parse_expression().parse_one_or_more(TokenType::TkComma);
        return RelStatementAst(c1, p1, (p2));
    }
    #[parser_rule]
    fn parse_inner_scope(&mut self) -> ParserReturnType<InnerScopeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkLeftCurlyBrace).parse_once()?;
        let p2 = self.parse_statement().parse_zero_or_more(TokenType::NoToken);
        let p3 = self.parse_token(TokenType::TkRightCurlyBrace).parse_once()?;
        return InnerScopeAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_statement(&mut self) -> ParserReturnType<StatementAst> {
        let p1 = self.parse_use_statement();
        let p2 = self.parse_let_statement();
        let p3 = self.parse_ret_statement();
        let p4 = self.parse_exit_statement();
        let p5 = self.parse_skip_statement();
        let p6 = self.parse_pin_statement();
        let p7 = self.parse_rel_statement();
        let p8 = self.parse_assignment_statement();
        let p9 = self.parse_expression();
        let p10 = (p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8 | p9).parse_once()?;
        return p10;
    }
    #[parser_rule]
    fn parse_global_use_statement(&mut self) -> ParserReturnType<UseStatementAst> {
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_use_statement().parse_once()?;
        p2.annotations = (p1);
        return p2;
    }
    #[parser_rule]
    fn parse_use_statement(&mut self) -> ParserReturnType<UseStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwUse).parse_once()?;
        let p2 = self.parse_upper_identifier().parse_once()?;
        let p3 = self.parse_generic_parameters().parse_optional().unwrap_or(GenericParameterGroupAst());
        let p4 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        return UseStatementAst(c1, (), p1, TypeAst::from(p2), p3, p4, p5);
    }
    #[parser_rule]
    fn parse_global_constant(&mut self) -> ParserReturnType<GlobalConstantAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_annotation().parse_zero_or_more(TokenType::TkNewLine);
        let p2 = self.parse_token(TokenType::KwCmp).parse_once()?;
        let p3 = self.parse_identifier().parse_once()?;
        let p4 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p5 = self.parse_type().parse_once()?;
        let p6 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p7 = self.parse_global_constant_value().parse_once()?;
        return GlobalConstantAst(c1, (p1), p2, p3, p4, p5, p6, p7);
    }
    #[parser_rule]
    fn parse_let_statement(&mut self) -> ParserReturnType<LetStatementAst> {
        let p1 = self.parse_let_statement_initialized();
        let p2 = self.parse_let_statement_uninitialized();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_let_statement_initialized(&mut self) -> ParserReturnType<LetStatementInitializedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwLet).parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p4 = self.parse_expression().parse_once()?;
        return LetStatementInitializedAst(c1, p1, p2, p3, p4);
    }
    #[parser_rule]
    fn parse_let_statement_uninitialized(&mut self) -> ParserReturnType<LetStatementUninitializedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwLet).parse_once()?;
        let p2 = self.parse_local_variable().parse_once()?;
        let p3 = self.parse_token(TokenType::TkColon).parse_once()?;
        let p4 = self.parse_type().parse_once()?;
        return LetStatementUninitializedAst(c1, p1, p2, p3, p4);
    }
    #[parser_rule]
    fn parse_local_variable(&mut self) -> ParserReturnType<LocalVariableAst> {
        let p1 = self.parse_local_variable_destructure_array();
        let p2 = self.parse_local_variable_destructure_tuple();
        let p3 = self.parse_local_variable_destructure_object();
        let p4 = self.parse_local_variable_single_identifier();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_local_variable_destructure_skip_argument(&mut self) -> ParserReturnType<LocalVariableDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkUnderscore).parse_once()?;
        return LocalVariableDestructureSkip1ArgumentAst(c1, p1);
    }
    #[parser_rule]
    fn parse_local_variable_destructure_skip_arguments(&mut self) -> ParserReturnType<LocalVariableDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDblDot).parse_once()?;
        let p2 = self.parse_local_variable_single_identifier().parse_optional();
        return LocalVariableDestructureSkipNArgumentsAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_local_variable_single_identifier(&mut self) -> ParserReturnType<LocalVariableSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwMut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_local_variable_single_identifier_alias().parse_optional();
        return LocalVariableSingleIdentifierAst(c1, p1, p2, p3);;
    }
    #[parser_rule]
    fn parse_local_variable_single_identifier_alias(&mut self) -> ParserReturnType<LocalVariableSingleIdentifierAliasAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwAs).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return LocalVariableSingleIdentifierAliasAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_local_variable_destructure_array(&mut self) -> ParserReturnType<LocalVariableDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBrackL).parse_once()?;
        let p2 = self.parse_local_variable_nested_for_destructure_array().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkBrackR).parse_once()?;
        return LocalVariableDestructureArrayAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_local_variable_destructure_tuple(&mut self) -> ParserReturnType<LocalVariableDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_local_variable_nested_for_destructure_tuple().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return LocalVariableDestructureTupleAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_local_variable_destructure_object(&mut self) -> ParserReturnType<LocalVariableDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p3 = self.parse_local_variable_nested_for_destructure_object().parse_zero_or_more(TokenType::TkComma);
        let p4 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return LocalVariableDestructureObjectAst(c1, p1, p2, (p3), p4);
    }
    #[parser_rule]
    fn parse_local_variable_attribute_binding(&mut self) -> ParserReturnType<LocalVariableAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p3 = self.parse_local_variable_nested_for_attribute_binding().parse_once()?;
        return LocalVariableAttributeBindingAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_array(&mut self) -> ParserReturnType<LocalVariableNestedForDestructureArrayAst> {
        let p1 = self.parse_local_variable_destructure_array();
        let p2 = self.parse_local_variable_destructure_tuple();
        let p3 = self.parse_local_variable_destructure_object();
        let p4 = self.parse_local_variable_single_identifier();
        let p5 = self.parse_local_variable_destructure_skip_arguments();
        let p6 = self.parse_local_variable_destructure_skip_argument();
        let p7 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p7;
    }
    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_tuple(&mut self) -> ParserReturnType<LocalVariableNestedForDestructureTupleAst> {
        let p1 = self.parse_local_variable_destructure_array();
        let p2 = self.parse_local_variable_destructure_tuple();
        let p3 = self.parse_local_variable_destructure_object();
        let p4 = self.parse_local_variable_single_identifier();
        let p5 = self.parse_local_variable_destructure_skip_arguments();
        let p6 = self.parse_local_variable_destructure_skip_argument();
        let p7 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p7;
    }
    #[parser_rule]
    fn parse_local_variable_nested_for_destructure_object(&mut self) -> ParserReturnType<LocalVariableNestedForDestructureObjectAst> {
        let p1 = self.parse_local_variable_attribute_binding();
        let p2 = self.parse_local_variable_single_identifier();
        let p3 = self.parse_local_variable_destructure_skip_arguments();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_local_variable_nested_for_attribute_binding(&mut self) -> ParserReturnType<LocalVariableNestedForAttributeBindingAst> {
        let p1 = self.parse_local_variable_destructure_array();
        let p2 = self.parse_local_variable_destructure_tuple();
        let p3 = self.parse_local_variable_destructure_object();
        let p4 = self.parse_local_variable_single_identifier();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_assignment_statement(&mut self) -> ParserReturnType<AssignmentStatementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_one_or_more(TokenType::TkComma);
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p3 = self.parse_expression().parse_one_or_more(TokenType::TkComma);
        return AssignmentStatementAst(c1, (p1), p2, (p3));
    }
    #[parser_rule]
    fn parse_case_expression_branch_simple(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_else_case();
        let p2 = self.parse_pattern_statement_flavour_else();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_case_expression_branch(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let p1 = self.parse_pattern_statement_flavour_destructuring();
        let p2 = self.parse_pattern_statement_flavour_non_destructuring();
        let p3 = self.parse_pattern_statement_flavour_else_case();
        let p4 = self.parse_pattern_statement_flavour_else();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_pattern_statement_flavour_destructuring(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwIs).parse_once()?;
        let p2 = self.parse_pattern_group_destructure().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_pattern_guard().parse_optional();
        let p4 = self.parse_inner_scope().parse_once()?;
        return CaseExpressionBranchAst(c1, p1, (p2), p3, p4);
    }
    #[parser_rule]
    fn parse_pattern_statement_flavour_non_destructuring(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_boolean_comparison_op().parse_once()?;
        let p2 = self.parse_pattern_variant_expression().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_inner_scope().parse_once()?;
        return CaseExpressionBranchAst(c1, p1, (p2), None, p3);
    }
    #[parser_rule]
    fn parse_pattern_statement_flavour_else_case(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else_case().parse_once()?;
        return CaseExpressionBranchAst.from_else_to_else_case(c1, p1);
    }
    #[parser_rule]
    fn parse_pattern_statement_flavour_else(&mut self) -> ParserReturnType<CaseExpressionBranchAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_pattern_variant_else().parse_once()?;
        let p2 = self.parse_inner_scope().parse_once()?;
        return CaseExpressionBranchAst(c1, None, ([p1]), None, p2);
    }
    #[parser_rule]
    fn parse_pattern_group_destructure(&mut self) -> ParserReturnType<PatternGroupDestructureAst> {
        let p1 = self.parse_pattern_variant_destructure_array();
        let p2 = self.parse_pattern_variant_destructure_tuple();
        let p3 = self.parse_pattern_variant_destructure_object();
        return (p1 | p2 | p3).parse_once()?;
    }
    #[parser_rule]
    fn parse_pattern_variant_skip_argument(&mut self) -> ParserReturnType<PatternVariantDestructureSkip1ArgumentAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkUnderscore).parse_once()?;
        return PatternVariantDestructureSkip1ArgumentAst(c1, p1);
    }
    #[parser_rule]
    fn parse_pattern_variant_skip_arguments(&mut self) -> ParserReturnType<PatternVariantDestructureSkipNArgumentsAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDblDot).parse_once()?;
        let p2 = self.parse_pattern_variant_single_identifier().parse_optional();
        return PatternVariantDestructureSkipNArgumentsAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_pattern_variant_single_identifier(&mut self) -> ParserReturnType<PatternVariantSingleIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwMut).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        let p3 = self.parse_local_variable_single_identifier_alias().parse_optional();
        return PatternVariantSingleIdentifierAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_pattern_variant_destructure_tuple(&mut self) -> ParserReturnType<PatternVariantDestructureTupleAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_pattern_variant_nested_for_destructure_tuple().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return PatternVariantDestructureTupleAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_pattern_variant_destructure_array(&mut self) -> ParserReturnType<PatternVariantDestructureArrayAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBrackL).parse_once()?;
        let p2 = self.parse_pattern_variant_nested_for_destructure_array().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkBrackR).parse_once()?;
        return PatternVariantDestructureArrayAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_pattern_variant_destructure_object(&mut self) -> ParserReturnType<PatternVariantDestructureObjectAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p3 = self.parse_pattern_variant_nested_for_destructure_object().parse_zero_or_more(TokenType::TkComma);
        let p4 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return PatternVariantDestructureObjectAst(c1, p1, p2, (p3), p4);
    }
    #[parser_rule]
    fn parse_pattern_variant_attribute_binding(&mut self) -> ParserReturnType<PatternVariantAttributeBindingAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p3 = self.parse_pattern_variant_nested_for_attribute_binding().parse_once()?;
        return PatternVariantAttributeBindingAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_pattern_variant_literal(&mut self) -> ParserReturnType<PatternVariantLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_literal_float();
        let p2 = self.parse_literal_integer();
        let p3 = self.parse_literal_string();
        let p4 = self.parse_literal_boolean();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return PatternVariantLiteralAst(c1, p5);
    }
    #[parser_rule]
    fn parse_pattern_variant_expression(&mut self) -> ParserReturnType<PatternVariantExpressionAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_expression().parse_once()?;
        return PatternVariantExpressionAst(c1, p1);
    }
    #[parser_rule]
    fn parse_pattern_variant_else(&mut self) -> ParserReturnType<PatternVariantElseAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwElse).parse_once()?;
        return PatternVariantElseAst(c1, p1);
    }
    #[parser_rule]
    fn parse_pattern_variant_else_case(&mut self) -> ParserReturnType<PatternVariantElseCaseAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwElse).parse_once()?;
        let p2 = self.parse_case_expression().parse_once()?;
        return PatternVariantElseCaseAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_tuple(&mut self) -> ParserReturnType<PatternVariantNestedForDestructureTupleAst> {
        let p1 = self.parse_pattern_variant_destructure_array();
        let p2 = self.parse_pattern_variant_destructure_tuple();
        let p3 = self.parse_pattern_variant_destructure_object();
        let p4 = self.parse_pattern_variant_single_identifier();
        let p5 = self.parse_pattern_variant_literal();
        let p6 = self.parse_pattern_variant_skip_arguments();
        let p7 = self.parse_pattern_variant_skip_argument();
        let p8 = (p1 | p2 | p3 | p4 | p5 | p6 | p7).parse_once()?;
        return p8;
    }
    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_array(&mut self) -> ParserReturnType<PatternVariantNestedForDestructureArrayAst> {
        let p1 = self.parse_pattern_variant_destructure_array();
        let p2 = self.parse_pattern_variant_destructure_tuple();
        let p3 = self.parse_pattern_variant_destructure_object();
        let p4 = self.parse_pattern_variant_single_identifier();
        let p5 = self.parse_pattern_variant_literal();
        let p6 = self.parse_pattern_variant_skip_arguments();
        let p7 = self.parse_pattern_variant_skip_argument();
        let p8 = (p1 | p2 | p3 | p4 | p5 | p6 | p7).parse_once()?;
        return p8;
    }
    #[parser_rule]
    fn parse_pattern_variant_nested_for_destructure_object(&mut self) -> ParserReturnType<PatternVariantNestedForDestructureObjectAst> {
        let p1 = self.parse_pattern_variant_attribute_binding();
        let p2 = self.parse_pattern_variant_single_identifier();
        let p3 = self.parse_pattern_variant_skip_arguments();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_pattern_variant_nested_for_attribute_binding(&mut self) -> ParserReturnType<PatternVariantNestedForAttributeBindingAst> {
        let p1 = self.parse_pattern_variant_destructure_array();
        let p2 = self.parse_pattern_variant_destructure_tuple();
        let p3 = self.parse_pattern_variant_destructure_object();
        let p4 = self.parse_pattern_variant_literal();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_pattern_guard(&mut self) -> ParserReturnType<PatternGuardAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwAnd).parse_once()?;
        let p2 = self.parse_expression().parse_once()?;
        return PatternGuardAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_1(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::KwOr).parse_once()?;
        return p1;
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_2(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::KwAnd).parse_once()?;
        return p1;
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_3(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::KwIs).parse_once()?;
        return p1;
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_4(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkEq);
        let p2 = self.parse_token(TokenType::TkNe);
        let p3 = self.parse_token(TokenType::TkLe);
        let p4 = self.parse_token(TokenType::TkGe);
        let p5 = self.parse_token(TokenType::TkLt);
        let p6 = self.parse_token(TokenType::TkGt);
        let p7 = self.parse_token(TokenType::TkSs);
        let p8 = (p1 | p2 | p3 | p4 | p5 | p6 | p7).parse_once()?;
        return p8;
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_5(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkAdd);
        let p2 = self.parse_token(TokenType::TkSub);
        let p3 = self.parse_token(TokenType::TkAddAssign);
        let p4 = self.parse_token(TokenType::TkSubAssign);
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_binary_op_precedence_level_6(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkMul);
        let p2 = self.parse_token(TokenType::TkDiv);
        let p3 = self.parse_token(TokenType::TkRem);
        let p4 = self.parse_token(TokenType::TkMod);
        let p5 = self.parse_token(TokenType::TkExp);
        let p6 = self.parse_token(TokenType::TkMulAssign);
        let p7 = self.parse_token(TokenType::TkDivAssign);
        let p8 = self.parse_token(TokenType::TkRemAssign);
        let p9 = self.parse_token(TokenType::TkModAssign);
        let p10 = self.parse_token(TokenType::TkExpAssign);
        let p11 = (p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8 | p9 | p10).parse_once()?;
        return p11;
    }
    #[parser_rule]
    fn parse_boolean_comparison_op(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkEq);
        let p2 = self.parse_token(TokenType::TkNe);
        let p3 = self.parse_token(TokenType::TkLe);
        let p4 = self.parse_token(TokenType::TkGe);
        let p5 = self.parse_token(TokenType::TkLt);
        let p6 = self.parse_token(TokenType::TkGt);
        let p8 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p8;
    }
    #[parser_rule]
    fn parse_unary_op(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_unary_op_async_call().parse_once()?;
        return p1;
    }
    #[parser_rule]
    fn parse_unary_op_async_call(&mut self) -> ParserReturnType<UnaryExpressionOperatorAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwAsync).parse_once()?;
        return UnaryExpressionOperatorAsyncAst(c1, p1);
    }
    #[parser_rule]
    fn parse_postfix_op(&mut self) -> ParserReturnType<PostfixExpressionOperatorAst> {
        let p1 = self.parse_postfix_op_function_call();
        let p2 = self.parse_postfix_op_member_access();
        let p3 = self.parse_postfix_op_early_return();
        let p4 = self.parse_postfix_op_not_keyword();
        let p5 = self.parse_postfix_op_step_keyword();
        let p6 = (p1 | p2 | p3 | p4 | p5).parse_once()?;
        return p6;
    }
    #[parser_rule]
    fn parse_postfix_op_function_call(&mut self) -> ParserReturnType<PostfixExpressionOperatorFunctionCallAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_generic_arguments().parse_optional().unwrap(GenericArgumentGroupAst());
        let p2 = self.parse_function_call_arguments().parse_once()?;
        let p3 = self.parse_token(TokenType::TkDblDot).parse_optional();
        return PostfixExpressionOperatorFunctionCallAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_postfix_op_member_access(&mut self) -> ParserReturnType<PostfixExpressionOperatorMemberAccessAst> {
        let p1 = self.parse_postfix_op_member_access_runtime();
        let p2 = self.parse_postfix_op_member_access_static();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_postfix_op_member_access_runtime(&mut self) -> ParserReturnType<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_identifier();
        let p3 = self.parse_lexeme(TokenType::LxDecInteger);
        let p4 = (p2 | p3).parse_once()?;
        return PostfixExpressionOperatorMemberAccessAst(c1, p1, p4);
    }
    #[parser_rule]
    fn parse_postfix_op_member_access_static(&mut self) -> ParserReturnType<PostfixExpressionOperatorMemberAccessAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDblColon).parse_once()?;
        let p2 = self.parse_identifier().parse_once()?;
        return PostfixExpressionOperatorMemberAccessAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_postfix_op_early_return(&mut self) -> ParserReturnType<PostfixExpressionOperatorEarlyReturnAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkQst).parse_once()?;
        return PostfixExpressionOperatorEarlyReturnAst(c1, p1);
    }
    #[parser_rule]
    fn parse_postfix_op_not_keyword(&mut self) -> ParserReturnType<PostfixExpressionOperatorNotKeywordAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_token(TokenType::KwNot).parse_once()?;
        return PostfixExpressionOperatorNotKeywordAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_postfix_op_step_keyword(&mut self) -> ParserReturnType<PostfixExpressionOperatorStepKeywordAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_token(TokenType::KwStep).parse_once()?;
        return PostfixExpressionOperatorStepKeywordAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_convention(&mut self) -> ParserReturnType<ConventionAst> {
        let p1 = self.parse_convention_mut();
        let p2 = self.parse_convention_ref();
        let p3 = self.parse_convention_mov();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_convention_mov(&mut self) -> ParserReturnType<ConventionMovAst> {
        let c1 = self.current_pos();
        return ConventionMovAst(c1);
    }
    #[parser_rule]
    fn parse_convention_ref(&mut self) -> ParserReturnType<ConventionRefAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBorrow).parse_once()?;
        return ConventionRefAst(c1, p1);
    }
    #[parser_rule]
    fn parse_convention_mut(&mut self) -> ParserReturnType<ConventionMutAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBorrow).parse_once()?;
        let p2 = self.parse_token(TokenType::KwMut).parse_once()?;
        return ConventionMutAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_object_initializer(&mut self) -> ParserReturnType<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_object_initializer_arguments().parse_once()?;
        return ObjectInitializerAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_object_initializer_arguments(&mut self) -> ParserReturnType<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_object_initializer_argument().parse_zero_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return ObjectInitializerArgumentGroupAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_object_initializer_argument(&mut self) -> ParserReturnType<ObjectInitializerArgumentAst> {
        let p1 = self.parse_object_initializer_argument_named();
        let p2 = self.parse_object_initializer_argument_unnamed();
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_object_initializer_argument_unnamed(&mut self) -> ParserReturnType<ObjectInitializerArgumentUnnamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDblDot).parse_optional();
        let p2 = self.parse_identifier().parse_once()?;
        return ObjectInitializerArgumentUnnamedAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_object_initializer_argument_named(&mut self) -> ParserReturnType<ObjectInitializerArgumentNamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p3 = self.parse_expression().parse_once()?;
        return ObjectInitializerArgumentNamedAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_type(&mut self) -> ParserReturnType<TypeAst> {
        let p1 = self.parse_type_optional();
        let p2 = self.parse_type_variant();
        let p3 = self.parse_type_tuple();
        let p4 = self.parse_type_single();
        let p5 = (p1 | p2 | p3 | p4).parse_once()?;
        return p5;
    }
    #[parser_rule]
    fn parse_type_optional(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkQst).parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        return TypeOptionalAst(c1, p1, p2).to_type();
    }
    #[parser_rule]
    fn parse_type_single(&mut self) -> ParserReturnType<TypeAst> {
        let p1 = self.parse_type_single_with_namespace();
        let p2 = self.parse_type_single_with_self();
        let p3 = self.parse_type_single_without_namespace_or_self();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_type_single_with_namespace(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_one_or_more(TokenType::TkDblColon);
        let p2 = self.parse_token(TokenType::TkDblColon).parse_once()?;
        let p3 = self.parse_generic_identifier().parse_one_or_more(TokenType::TkDblColon);
        return TypeAst(c1, (p1), (p3));
    }
    #[parser_rule]
    fn parse_type_single_with_self(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_self_type_keyword().parse_once()?;
        let p2 = self.parse_types_after_self().parse_optional().unwrap_or(());
        return TypeAst(c1, (), ([p1]) + p2);
    }
    #[parser_rule]
    fn parse_types_after_self(&mut self) -> Vec<GenericIdentifierAst> {
        let p1 = self.parse_token(TokenType::TkDblColon);
        let p2 = self.parse_generic_identifier().parse_zero_or_more(TokenType::TkDblColon);
        return p2;
    }
    #[parser_rule]
    fn parse_type_single_without_namespace_or_self(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_generic_identifier().parse_one_or_more(TokenType::TkDblColon);
        return TypeAst(c1, (), p1);
    }
    #[parser_rule]
    fn parse_self_type_keyword(&mut self) -> ParserReturnType<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwSelfType).parse_once()?;
        return GenericIdentifierAst(c1, p1.token.token_metadata);
    }
    #[parser_rule]
    fn parse_type_tuple(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_type().parse_zero_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return TypeTupleAst(c1, p1, (p2), p3).to_type();
    }
    #[parser_rule]
    fn parse_type_non_union(&mut self) -> ParserReturnType<TypeAst> {
        let p1 = self.parse_type_single();
        let p2 = self.parse_type_tuple();
        let p3 = self.parse_type_optional();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_type_variant(&mut self) -> ParserReturnType<TypeAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_non_union().parse_two_or_more(TokenType::TkUnion);
        return TypeVariantAst(c1, (p1)).to_type();
    }
    #[parser_rule]
    fn parse_identifier(&mut self) -> ParserReturnType<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme(TokenType::LxIdentifier).parse_once()?;
        return IdentifierAst(c1, p1.token.token_metadata);
    }
    #[parser_rule]
    fn parse_upper_identifier(&mut self) -> ParserReturnType<IdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme(TokenType::LxUpperIdentifier).parse_once()?;
        return IdentifierAst(c1, p1.token.token_metadata);
    }
    #[parser_rule]
    fn parse_generic_identifier(&mut self) -> ParserReturnType<GenericIdentifierAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_upper_identifier().parse_once()?;
        let p2 = self.parse_generic_arguments().parse_optional().unwrap_or(GenericArgumentGroupAst());
        return GenericIdentifierAst(c1, p1.value, p2);
    }
    #[parser_rule]
    fn parse_literal(&mut self) -> ParserReturnType<LiteralAst> {
        let p1 = self.parse_literal_float();
        let p2 = self.parse_literal_integer();
        let p3 = self.parse_literal_string();
        let p4 = self.parse_literal_tuple(self.parse_expression);
        let p5 = self.parse_literal_array(self.parse_expression);
        let p6 = self.parse_literal_boolean();
        let p7 = (p1 | p2 | p3 | p4 | p5 | p6).parse_once()?;
        return p7;
    }
    #[parser_rule]
    fn parse_literal_float(&mut self) -> ParserReturnType<FloatLiteralAst> {
        let p1 = self.parse_literal_float_b10().parse_once()?;
        return p1;
    }
    #[parser_rule]
    fn parse_literal_integer(&mut self) -> ParserReturnType<IntegerLiteralAst> {
        let p1 = self.parse_literal_integer_b10();
        let p2 = self.parse_literal_integer_b02();
        let p3 = self.parse_literal_integer_b16();
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_literal_string(&mut self) -> ParserReturnType<StringLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_lexeme(TokenType::LxDoubleQuoteStr).parse_once()?;
        return StringLiteralAst(c1, p1);
    }
    #[parser_rule]
    fn parse_literal_tuple<T>(self, item: Box<dyn ParserRuleHandler<T>>) -> ParserReturnType<TupleLiteralAst> {
        let p1 = self.parse_literal_tuple_0_items();
        let p2 = self.parse_literal_tuple_1_items(self.parse_expression);
        let p3 = self.parse_literal_tuple_n_items(self.parse_expression);
        let p4 = (p1 | p2 | p3).parse_once()?;
        return p4;
    }
    #[parser_rule]
    fn parse_literal_array(self, item: Box<dyn ParserRuleHandler<T>>) -> ParserReturnType<ArrayLiteralAst> {
        let p1 = self.parse_literal_array_0_items();
        let p2 = self.parse_literal_array_n_items(item);
        let p3 = (p1 | p2).parse_once()?;
        return p3;
    }
    #[parser_rule]
    fn parse_literal_boolean(&mut self) -> ParserReturnType<BooleanLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::KwTrue);
        let p2 = self.parse_token(TokenType::KwFalse);
        let p3 = (p1 | p2).parse_once()?;
        return BooleanLiteralAst(c1, p3);
    }
    #[parser_rule]
    fn parse_literal_float_b10(&mut self) -> ParserReturnType<FloatLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme(TokenType::LxDecInteger).parse_once()?;
        let p3 = self.parse_token(TokenType::TkDot).parse_once()?;
        let p4 = self.parse_lexeme(TokenType::LxDecInteger).parse_once()?;
        let p5 = self.parse_float_postfix_type().parse_optional();
        return FloatLiteralAst(c1, p1, p2, p3, p4, p5);
    }
    #[parser_rule]
    fn parse_literal_integer_b10(&mut self) -> ParserReturnType<IntegerLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme(TokenType::LxDecInteger).parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return IntegerLiteralAst(c1, p1, p2, TypeAst::from(p3) if p3 else p3);
    }
    #[parser_rule]
    fn parse_literal_integer_b02(&mut self) -> ParserReturnType<IntegerLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme(TokenType::LxBinDigits).parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return IntegerLiteralAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_literal_integer_b16(&mut self) -> ParserReturnType<IntegerLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_numeric_prefix_op().parse_optional();
        let p2 = self.parse_lexeme(TokenType::LxHexDigits).parse_once()?;
        let p3 = self.parse_integer_postfix_type().parse_optional();
        return IntegerLiteralAst(c1, p1, p2, p3);
    }
    #[parser_rule]
    fn parse_numeric_prefix_op(&mut self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkSub);
        let p2 = self.parse_token(TokenType::TkAdd);
        let p3 = (p1 | p2).parse_once()?
        return p3;
    }
    #[parser_rule]
    fn parse_integer_postfix_type(&mut self) -> SppTokenType {
        let p1 = self.parse_token(TokenType::TkUnderscore).parse_once()?;
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
        let p14 = (p2 | p3 | p4 | p5 | p6 | p7 | p8 | p9 | p10 | p11 | p12 | p13).parse_once()?;
        return p14;
    }
    #[parser_rule]
    fn parse_float_postfix_type(&mut self) -> SppTokenType {
        let p1 = self.parse_token(TokenType::TkUnderscore).parse_once()?;
        let p2 = self.parse_characters("f8");
        let p3 = self.parse_characters("f16");
        let p4 = self.parse_characters("f32");
        let p5 = self.parse_characters("f64");
        let p6 = self.parse_characters("f128");
        let p7 = self.parse_characters("f256");
        let p8 = (p2 | p3 | p4 | p5 | p6 | p7).parse_once()?;
        return p8;
    }
    #[parser_rule]
    fn parse_literal_tuple_0_items(&mut self) -> ParserReturnType<TupleLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return TupleLiteralAst(c1, p1, (), p2);
    }
    #[parser_rule]
    fn parse_literal_tuple_1_items<T>(self, item: Box<dyn ParserRuleHandler<T>>) -> ParserReturnType<TupleLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = item().parse_once()?;
        let p3 = self.parse_token(TokenType::TkComma).parse_once()?;
        let p4 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return TupleLiteralAst(c1, p1, ([p2]), p4);
    }
    #[parser_rule]
    fn parse_literal_tuple_n_items<T>(self, item: Box<dyn ParserRuleHandler<T>>) -> ParserReturnType<TupleLiteralAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = item().parse_two_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return TupleLiteralAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_literal_array_0_items(&mut self) -> ParserReturnType<ArrayLiteral0ElementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBrackL).parse_once()?;
        let p2 = self.parse_type().parse_once()?;
        let p3 = self.parse_token(TokenType::TkComma).parse_once()?;
        let p4 = self.parse_lexeme(TokenType::LxDecInteger).parse_once()?;
        let p5 = self.parse_token(TokenType::TkBrackR).parse_once()?;
        return ArrayLiteral0ElementAst(c1, p1, p2, p3, p4, p5);
    }
    #[parser_rule]
    fn parse_literal_array_n_items<T>(self, item: Box<dyn ParserRuleHandler<T>>) -> ParserReturnType<ArrayLiteralNElementAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkBrackL).parse_once()?;
        let p2 = item().parse_one_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkBrackR).parse_once()?;
        return ArrayLiteralNElementAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_global_constant_value(&mut self) -> ParserReturnType<ExpressionAst> {
        let p1 = self.parse_literal_float();
        let p2 = self.parse_literal_integer();
        let p3 = self.parse_literal_string();
        let p4 = self.parse_literal_tuple(self.parse_global_constant_value);
        let p5 = self.parse_literal_array(self.parse_global_constant_value);
        let p6 = self.parse_literal_boolean();
        let p7 = self.parse_global_object_initializer();
        let p8 = self.parse_identifier();
        let p9 = (p1 | p2 | p3 | p4 | p5 | p6 | p7 | p8).parse_once()?;
        return p9;
    }
    #[parser_rule]
    fn parse_global_object_initializer(&mut self) -> ParserReturnType<ObjectInitializerAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_type_single().parse_once()?;
        let p2 = self.parse_global_object_initializer_arguments().parse_once()?;
        return ObjectInitializerAst(c1, p1, p2);
    }
    #[parser_rule]
    fn parse_global_object_initializer_arguments(&mut self) -> ParserReturnType<ObjectInitializerArgumentGroupAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkParenL).parse_once()?;
        let p2 = self.parse_global_object_initializer_argument_named().parse_zero_or_more(TokenType::TkComma);
        let p3 = self.parse_token(TokenType::TkParenR).parse_once()?;
        return ObjectInitializerArgumentGroupAst(c1, p1, (p2), p3);
    }
    #[parser_rule]
    fn parse_global_object_initializer_argument_named(&mut self) -> ParserReturnType<ObjectInitializerArgumentNamedAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_identifier().parse_once()?;
        let p2 = self.parse_token(TokenType::TkAssign).parse_once()?;
        let p3 = self.parse_global_constant_value().parse_once()?;
        return ObjectInitializerArgumentNamedAst(c1, p1, p2, p3);
    }

    #[parser_rule]
    fn parse_keyword(&mut self, keyword: String) -> ParserReturnType<TokenAst> {
        let c1 = self.current_pos();
        for c in KEYWORD_STRINGS[keyword].chars() {
            let p1 = self.parse_character(c);
        }
        return TokenAst::new(c1, "");
    }

    #[parser_rule]
    fn parse_token_left_curly_brace(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkLeftCurlyBrace).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_right_curly_brace(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkRightCurlyBrace).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_left_parenthesis(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkLeftParenthesis).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_right_parenthesis(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkRightParenthesis).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_left_square_bracket(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkLeftSquareBracket).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_right_square_bracket(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkRightSquareBracket).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_comma(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkComma).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_colon(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkColon).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_assign(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkEqualsSign).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_newline(&self) -> ParserReturnType<TokenAst> {
        let p1 = self.parse_token(TokenType::TkNewLine).parse_once()?;
        return p1;
    }

    #[parser_rule]
    fn parse_token_rightward_arrow(&self) -> ParserReturnType<TokenAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkHyphenMinus).parse_once()?;
        let p2 = self.parse_token(TokenType::TkGreaterThanSign).parse_once()?;
        return TokenAst::new(c1, "");
    }

    #[parser_rule]
    fn parse_token_double_dot(&self) -> ParserReturnType<TokenAst> {
        let c1 = self.current_pos();
        let p1 = self.parse_token(TokenType::TkDot).parse_once()?;
        let p2 = self.parse_token(TokenType::TkDot).parse_once()?;
        return TokenAst::new(c1, "");
    }

    #[parser_rule]
    fn parse_eof(&self, token_type: TokenType) -> ParserReturnType<TokenAst> {}

    #[parser_rule]
    fn parse_lexeme(&self, token_type: TokenType) -> ParserReturnType<TokenAst> {}

    #[parser_rule]
    fn parse_token_primitive(&self, token_type: TokenType) -> ParserReturnType<TokenAst> {}
}
