extern crate spp;
extern crate proc_macros;

use crate::spp::spp::lexer::lexer::Lexer;
use crate::spp::spp::parser::parser::Parser;
use crate::spp::spp::utilities::error_formatter::ErrorFormatter;
use proc_macros::should_parse_pass;

#[test]
#[should_parse_pass]
fn parse_class_prototype() {
    "
    cls MyClass { }
    "
}

#[test]
#[should_parse_pass]
fn parse_class_attribute() {
    "
    cls MyClass {
        my_attr_1: I32
        my_attr_2: I32
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_class_attribute_default_value() {
    "
    cls MyClass {
        my_attr_2: Arr[Str, 1] = [\"Hello\"]
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_sup_extension_prototype() {
    "
    sup MyClass ext Copy { }
    "
}

#[test]
#[should_parse_pass]
fn parse_sup_functions_prototype() {
    "
    sup MyClass { }
    "
}

#[test]
#[should_parse_pass]
fn parse_sup_method_prototype() {
    "
    sup MyClass {
        fun my_method_1() -> Void { }
        fun my_method_2() -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_sup_use_statement() {
    "
    sup MyClass {
        use NewType = OldType
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_subroutine_prototype() {
    "
    fun my_function() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_coroutine_prototype() {
    "
    cor my_coroutine() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_call_arguments_named() {
    "
    fun my_function() -> Void {
        other_function(arg1=other_thing, arg2=2)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_call_arguments_unnamed() {
    "
    fun my_function() -> Void {
        other_function(1, 2)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_call_no_arguments() {
    "
    fun my_function() -> Void {
        other_function()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_call_arguments() {
    "
    fun my_function() -> Void {
        other_function(1, arg=false)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_self_parameter_mov() {
    "
    sup MyClass {
        fun my_method(self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_self_parameter_mut_mov() {
    "
    sup MyClass {
        fun my_method(mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_self_parameter_mut() {
    "
    sup MyClass {
        fun my_method(&mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_self_parameter_ref() {
    "
    sup MyClass {
        fun my_method(&self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_required_parameters() {
    "
    fun my_function(arg1: I32, arg2: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_optional_parameters() {
    "
    fun my_function(arg2: I32 = 0) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_variadic_parameters() {
    "
    fun my_function(arg1: I32, ..arg2: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_function_parameters() {
    "
    fun my_function(arg1: I32, arg2: I32 = 0, ..arg3: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_argument_type_named() {
    "
    fun my_function() -> Void {
        other_function[T=I32, U=Str]()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_argument_type_unnamed() {
    "
    fun my_function() -> Void {
        other_function[I32, Str]()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_argument_comp_named() {
    "
    fun my_function() -> Void {
        other_function[n=1, m=2]()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_argument_comp_unnamed() {
    "
    fun my_function() -> Void {
        other_function[1, 2]()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_arguments() {
    "
    fun my_function() -> Void {
        other_function[I32, Str, T=Bool, n=1]()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_type_required() {
    "
    fun my_function[T, U]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_type_optional() {
    "
    fun my_function[T=I32, U=Str]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_type_variadic() {
    "
    fun my_function[T, ..U]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_type() {
    "
    fun my_function[T, U=I32, ..V]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_comp_required() {
    "
    fun my_function[cmp n: I32, cmp m: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_comp_optional() {
    "
    fun my_function[cmp n: I32=1, cmp m: I32=2]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_comp_variadic() {
    "
    fun my_function[cmp ..m: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameter_comp() {
    "
    fun my_function[cmp n: I32, cmp m: I32=1, cmp ..o: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_parameters() {
    "
    fun my_function[cmp n: I32, T, cmp m: I32=1, U=Str, cmp ..o: I32, ..V]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_generic_inline_constraints() {
    "
    fun my_function[T: Copy, U: Clone]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_where_block_constraints() {
    "
    fun my_function[T, U]() -> Void where [T, U: Copy, U: Clone] { }
    "
}

#[test]
#[should_parse_pass]
fn parse_annotations() {
    "
    @annotation1
    @annotation2
    fun my_function() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_1() {
    "
    fun my_function() -> Void {
        variable or other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_2() {
    "
    fun my_function() -> Void {
        variable and another_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_3() {
    "
    fun my_function() -> Void {
        variable is Destructure(a=1, b, ..)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_4() {
    "
    fun my_function() -> Void {
        variable == other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_5() {
    "
    fun my_function() -> Void {
        variable + other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_binary_expression_precedence_6() {
    "
    fun my_function() -> Void {
        variable * other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_unary_expression_async_op() {
    "
    fun my_function() -> Void {
        async function()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_function_call() {
    "
    fun my_function() -> Void {
        function()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_member_access_runtime() {
    "
    fun my_function() -> Void {
        variable.field
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_member_access_runtime_numeric() {
    "
    fun my_function() -> Void {
        tuple.0
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_member_access_static() {
    "
    fun my_function() -> Void {
        Type::method()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_early_return() {
    "
    fun my_function() -> Void {
        function()?
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_not_keyword() {
    "
    fun my_function() -> Void {
        variable.not
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_postfix_expression_step_keyword() {
    "
    fun my_function() -> Void {
        generator.step
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_parenthesized_expression() {
    "
    fun my_function() -> Void {
        (variable)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_self_identifier() {
    "
    fun my_function() -> Void {
        self
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_fold_expression() {
    "
    fun my_function() -> Void {
        tuple + ..
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_expression_patterns() {
    "
    fun my_function() -> Void {
        case my_tuple {
            (1, 2, a) { }
            (3, b, 4) { }
            (c, 5, 6) { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_expression_patterns_simple() {
    "
    fun my_function() -> Void {
        case my_tuple == (1, 2, 3) {
        }
        else case some_other_expression() {
        }
        else {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_loop_expression_boolean_condition() {
    "
    fun my_function() -> Void {
        loop true {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_loop_expression_iterable_condition() {
    "
    fun my_function() -> Void {
        loop i in some_vector {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_loop_expression_else_block() {
    "
    fun my_function() -> Void {
        loop i in some_vector {
        }
        else {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_gen_no_expression() {
    "
    fun my_function() -> Void {
        gen
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_gen_mov_expression() {
    "
    fun my_function() -> Void {
        gen variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_gen_ref_expression() {
    "
    fun my_function() -> Void {
        gen &variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_gen_mut_expression() {
    "
    fun my_function() -> Void {
        gen &mut variable
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_gen_expression_unroll() {
    "
    fun my_function() -> Void {
        gen with another_generator
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_with_expression() {
    "
    fun my_function() -> Void {
        with some_function_call() {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_with_expression_alias() {
    "
    fun my_function() -> Void {
        with some_function_call() as alias {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_ret_statement_no_value() {
    "
    fun my_function() -> Void {
        ret
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_ret_statement_value() {
    "
    fun my_function() -> Void {
        ret 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_exit_statement_no_value() {
    "
    fun my_function() -> Void {
        loop true {
            loop true {
                exit exit
            }
        }
    }"
}

#[test]
#[should_parse_pass]
fn parse_exit_statement_value() {
    "
    fun my_function() -> Void {
        loop true {
            loop true {
                exit exit 1
            }
        }
    }"
}

#[test]
#[should_parse_pass]
fn parse_exit_statement_skip() {
    "
    fun my_function() -> Void {
        loop true {
            loop true {
                exit skip
            }
        }
    }"
}

#[test]
#[should_parse_pass]
fn parse_skip_statement() {
    "
    fun my_function() -> Void {
        loop true {
            loop true {
                skip skip
            }
        }
    }"
}

#[test]
#[should_parse_pass]
fn parse_inner_scope() {
    "
    fun my_function() -> Void {
        {
            inner_function()
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_global_use_statement() {
    "
    use MyString = Str
    "
}

#[test]
#[should_parse_pass]
fn parse_global_constant() {
    "
    cmp constant: I32 = 1
    "
}

#[test]
#[should_parse_pass]
fn parse_let_statement_initialized() {
    "
    fun my_function() -> Void {
        let a = 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_let_statement_initialized_explicit_type() {
    "
    fun my_function() -> Void {
        let a: std::number::bigint::BigInt = 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_let_statement_uninitialized() {
    "
    fun my_function() -> Void {
        let a: I32
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_with_single_skip() {
    "
    fun my_function() -> Void {
        let (a, _, b, _) = tuple
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_with_multiple_skip() {
    "
    fun my_function() -> Void {
        let (a, .., b) = tuple
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_with_single_identifier_alias() {
    "
    fun my_function() -> Void {
        let MyType(attr as a) = object
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_single_identifier() {
    "
    fun my_function() -> Void {
        let a = 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_array() {
    "
    fun my_function() -> Void {
        let [a, b, c] = array
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_tuple() {
    "
    fun my_function() -> Void {
        let (a, b, c) = tuple
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_object() {
    "
    fun my_function() -> Void {
        let MyType(a, b, c) = object
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_local_variable_destructure_object_attr_binding() {
    "
    fun my_function() -> Void {
        let MyType(attr1=Point(x, y), attr2) = object
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_assignment_statement() {
    "
    fun my_function() -> Void {
        variable = 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_assignment_multiple_statement() {
    "
    fun my_function() -> Void {
        a, b, c = 1, 2, 3
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_else_pattern() {
    "
    fun my_function() -> Void {
        case value {
        }
        else {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_else_case_pattern_with_condition() {
    "
    fun my_function() -> Void {
        case value == 1 {
        }
        else case value == 2 {
        }
        else case value == 3 {
        }
        else {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_array() {
    "
    fun my_function() -> Void {
        case value of {
            [a, b, c] { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_tuple() {
    "
    fun my_function() -> Void {
        case value of {
            (a, b, c) { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_object() {
    "
    fun my_function() -> Void {
        case value of {
            MyType(a, b, c) { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_object_attr_binding() {
    "
    fun my_function() -> Void {
        case value of {
            MyType(attr1=Point(x, y), attr2) { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_literal() {
    "
    fun my_function() -> Void {
        case value of {
            1 { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_case_destructure_expression() {
    "
    fun my_function() -> Void {
        case array of {
            == some_function_call() { }
            == other_function_call() { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_pattern_guard() {
    "
    fun my_function() -> Void {
        case value of {
            1 and some_condition() { }
            2 and other_condition() { }
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_convention_mutable_borrow() {
    "
    fun my_function(a: &mut I32) -> Void {
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_convention_immutable_borrow() {
    "
    fun my_function(a: &I32) -> Void {
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_object_initializer_argument_named() {
    "
    fun my_function() -> Void {
        MyType(attr1=1, attr2=2)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_object_initializer_argument_unnamed() {
    "
    fun my_function() -> Void {
        MyType(attr1, attr2)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_object_initializer_argument_default() {
    "
    fun my_function() -> Void {
        MyType(1, ..other)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_object_initializer_arguments() {
    "
    fun my_function() -> Void {
        MyType(attr1, attr2=other, ..other)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_optional() {
    "
    fun my_function() -> Void {
        let a: I32?
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_tuple() {
    "
    fun my_function() -> Void {
        let a: (I32, I32)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_union() {
    "
    fun my_function() -> Void {
        let a: I32 or Str
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_intersection() {
    "
    fun my_function() -> Void {
        let a: I32 and Str
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_single() {
    "
    fun my_function() -> Void {
        let a: I32
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_with_namespace() {
    "
    fun my_function() -> Void {
        let a: std::inner::Str
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_nested_type() {
    "
    fun my_function() -> Void {
        let a: std::inner::Str::ValueType::Other
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_with_self() {
    "
    fun my_function() -> Void {
        let a: Self
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_type_with_self_nested() {
    "
    fun my_function() -> Void {
        let a: Self::InnerType
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_float() {
    "
    fun my_function() -> Void {
        let a = 1.0
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_integer() {
    "
    fun my_function() -> Void {
        let a = 1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_integer_with_sign() {
    "
    fun my_function() -> Void {
        let a = -1
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_integer_with_type() {
    "
    fun my_function() -> Void {
        let a = 1_u64
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_integer_base_2() {
    "
    fun my_function() -> Void {
        let a = 0b101
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_integer_base_16() {
    "
    fun my_function() -> Void {
        let a = 0x1F
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_string() {
    "
    fun my_function() -> Void {
        let a = \"string\"
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_boolean() {
    "
    fun my_function() -> Void {
        let a = true
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_literal_tuple_0_items() {
    "
    fun my_function() -> Void {
        let a = ()
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_tuple_1_item() {
    "
    fun my_function() -> Void {
        let a = (1,)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_tuple_n_items() {
    "
    fun my_function() -> Void {
        let a = (1, 2, 3)
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_array_0_items() {
    "
    fun my_function() -> Void {
        let a = [I32, 8]
    }
    "
}

#[test]
#[should_parse_pass]
fn parse_array_n_items() {
    "
    fun my_function() -> Void {
        let a = [1, 2, 3]
    }
    "
}
