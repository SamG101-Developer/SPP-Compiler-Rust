extern crate core_lib;
extern crate proc_macros;

use core_lib::lexer::lexer::Lexer;
use core_lib::parser::parser::Parser;
use proc_macros::{should_parse_fail, should_parse_pass};

#[test]
#[should_parse_pass]
fn test_class_prototype() {
    "
    @public
    cls MyClass { }
    "
}

#[test]
#[should_parse_pass]
fn test_class_attribute() {
    "
    @public
    cls MyClass {
        @public my_attr_1: I32
        @public my_attr_2: I32
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
        @public fun my_method_1() -> Void { }
        @public fun my_method_2() -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_sup_use_statement() {
    "
    sup MyClass {
        @public use NewType = OldType
    }
    "
}

#[test]
#[should_parse_pass]
fn test_subroutine_prototype() {
    "
    @public
    fun my_function() -> Void { }
    "
}

#[test]
#[should_parse_pass]
fn test_coroutine_prototype() {
    "
    @public
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
        @public fun my_method(self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_mut_mov() {
    "
    sup MyClass {
        @public fun my_method(mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_mut() {
    "
    sup MyClass {
        @public fun my_method(&mut self) -> Void { }
    }
    "
}

#[test]
#[should_parse_pass]
fn test_function_self_parameter_ref() {
    "
    sup MyClass {
        @public fun my_method(&self) -> Void { }
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
