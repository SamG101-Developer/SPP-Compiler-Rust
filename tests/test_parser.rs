extern crate core_lib;
extern crate proc_macros;

use core_lib::parser::parser::Parser;
use core_lib::lexer::lexer::Lexer;
use proc_macros::{should_parse_pass, should_parse_fail};


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
        @public my_attr_1: I32;
        @public my_attr_2: I32;
    }
    "
}
