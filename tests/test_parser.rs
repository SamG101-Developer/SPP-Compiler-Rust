extern crate core_lib;
extern crate proc_macros;

use core_lib::lexer::lexer::Lexer;
use core_lib::parser::parser::Parser;
use proc_macros::{should_parse_fail, should_parse_pass};

#[test]
#[should_parse_pass]
fn test_class_prototype() {
    "
    cls MyClass { }
    "
}

#[test]
#[should_parse_pass]
fn test_class_attribute() {
    "
    cls MyClass {
        my_attr_1: I32
        my_attr_2: I32
    }
    "
}

#[test]
#[should_parse_pass]
fn test_sup_extension_prototype() {
    "
    sup MyClass ext Copy { }
    "
}

#[test]
#[should_parse_pass]
fn test_sup_functions_prototype() {
    "
    sup MyClass { }
    "
}

#[test]
#[should_parse_pass]
fn test_sup_method_prototype() {
    "
    sup MyClass {
        fun my_method_1() -> Void { }
        fun my_method_2() -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_sup_use_statement() {
    "
    sup MyClass {
        use NewType = OldType
    }
    "
}

#[test]
#[should_parse_pass]
fn test_subroutine_prototype() {
    "
    fun my_function() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_coroutine_prototype() {
    "
    cor my_coroutine() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_function_call_arguments_named() {
    "
    fun my_function() -> Void {
        other_function(arg1=1, arg2=2)
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_call_arguments_unnamed() {
    "
    fun my_function() -> Void {
        other_function(1, 2)
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_call_no_arguments() {
    "
    fun my_function() -> Void {
        other_function()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_call_arguments() {
    "
    fun my_function() -> Void {
        other_function(1, arg=2)
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_mov() {
    "
    sup MyClass {
        fun my_method(self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_mut_mov() {
    "
    sup MyClass {
        fun my_method(mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_mut() {
    "
    sup MyClass {
        fun my_method(&mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_ref() {
    "
    sup MyClass {
        fun my_method(&self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_required_parameters() {
    "
    fun my_function(arg1: I32, arg2: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_function_optional_parameters() {
    "
    fun my_function(arg2: I32 = 0) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_function_variadic_parameters() {
    "
    fun my_function(arg1: I32, ..arg2: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_function_parameters() {
    "
    fun my_function(arg1: I32, arg2: I32 = 0, ..arg3: I32) -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_argument_type_named() {
    "
    fun my_function() -> Void {
        other_function[T=I32, U=Str]()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_argument_type_unnamed() {
    "
    fun my_function() -> Void {
        other_function[I32, Str]()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_argument_comp_named() {
    "
    fun my_function() -> Void {
        other_function[N=1, M=2]()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_argument_comp_unnamed() {
    "
    fun my_function() -> Void {
        other_function[1, 2]()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_arguments() {
    "
    fun my_function() -> Void {
        other_function[I32, Str, T=Bool, N=1]()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_type_required() {
    "
    fun my_function[T, U]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_type_optional() {
    "
    fun my_function[T=I32, U=Str]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_type_variadic() {
    "
    fun my_function[T, ..U]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_type() {
    "
    fun my_function[T, U=I32, ..V]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_comp_required() {
    "
    fun my_function[cmp N: I32, cmp M: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_comp_optional() {
    "
    fun my_function[cmp N: I32=1, cmp M: I32=2]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_comp_variadic() {
    "
    fun my_function[..cmp M: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameter_comp() {
    "
    fun my_function[cmp N: I32, cmp M: I32=1, ..cmp O: I32]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_parameters() {
    "
    fun my_function[cmp N: I32, T, cmp M: I32=1, U=Str, ..cmp O: I32, ..V]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_generic_inline_constraints() {
    "
    fun my_function[T: Copy, U: Clone]() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_where_block_constraints() {
    "
    fun my_function[T, U]() -> Void where [T, U: Copy, U: Clone] { }
    "
}

#[test]
#[should_parse_pass]
fn test_annotations() {
    "
    @annotation1
    @annotation2
    fun my_function() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_1() {
    "
    fun my_function() -> Void {
        variable or other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_2() {
    "
    fun my_function() -> Void {
        variable and another_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_3() {
    "
    fun my_function() -> Void {
        variable is Destructure(a=1, b, ..)
    }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_4() {
    "
    fun my_function() -> Void {
        variable == other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_5() {
    "
    fun my_function() -> Void {
        variable + other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_binary_expression_precedence_6() {
    "
    fun my_function() -> Void {
        variable * other_variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_unary_expression_async_op() {
    "
    fun my_function() -> Void {
        async function()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_function_call() {
    "
    fun my_function() -> Void {
        function()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_member_access_runtime() {
    "
    fun my_function() -> Void {
        variable.field
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_member_access_runtime_numeric() {
    "
    fun my_function() -> Void {
        tuple.0
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_member_access_static() {
    "
    fun my_function() -> Void {
        Type::method()
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_early_return() {
    "
    fun my_function() -> Void {
        function()?
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_not_keyword() {
    "
    fun my_function() -> Void {
        variable.not
    }
    "
}

#[test]
#[should_parse_pass]
fn test_postfix_expression_step_keyword() {
    "
    fun my_function() -> Void {
        generator.step
    }
    "
}

#[test]
#[should_parse_pass]
fn test_parenthesized_expression() {
    "
    fun my_function() -> Void {
        (variable)
    }
    "
}

#[test]
#[should_parse_pass]
fn test_self_identifier() {
    "
    fun my_function() -> Void {
        self
    }
    "
}

#[test]
#[should_parse_pass]
fn test_fold_expression() {
    "
    fun my_function() -> Void {
        tuple + ..
    }
    "
}

#[test]
#[should_parse_pass]
fn test_case_expression_patterns() {
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
fn test_case_expression_patterns_simple() {
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
fn test_loop_expression_boolean_condition() {
    "
    fun my_function() -> Void {
        loop true {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_loop_expression_iterable_condition() {
    "
    fun my_function() -> Void {
        loop i in some_vector {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_loop_expression_else_block() {
    "
    fun my_function() {
        loop i in some_vector {
        }
        else {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_gen_no_expression() {
    "
    fun my_function() -> Void {
        gen
    }
    "
}

#[test]
#[should_parse_pass]
fn test_gen_expression() {
    "
    fun my_function() -> Void {
        gen &mut variable
    }
    "
}

#[test]
#[should_parse_pass]
fn test_gen_expression_unroll() {
    "
    fun my_function() -> Void {
        gen with another_generator
    }
    "
}

#[test]
#[should_parse_pass]
fn test_with_expression() {
    "
    fun my_function() -> Void {
        with some_function_call() {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_with_expression_alias() {
    "
    fun my_function() -> Void {
        with some_function_call() as alias {
        }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_ret_statement_no_value() {
    "
    fun my_function() -> Void {
        ret
    }
    "
}

#[test]
#[should_parse_pass]
fn test_ret_statement_value() {
    "
    fun my_function() -> Void {
        ret 1
    }
    "
}

#[test]
#[should_parse_pass]
fn test_exit_statement_no_value() {
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
fn test_exit_statement_value() {
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
fn test_exit_statement_skip() {
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
fn test_skip_statement() {
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
fn parse_pin_statement() {
    "
    fun my_function() -> Void {
        pin a, b, c
    }
    "
}

#[test]
#[should_parse_pass]
fn test_rel_statement() {
    "
    fun my_function() -> Void {
        rel a, b, c
    }
    "
}

#[test]
#[should_parse_pass]
fn test_inner_scope() {
    "
    fun my_function() -> Void {
        {
            inner_function()
        }
    }
    "
}