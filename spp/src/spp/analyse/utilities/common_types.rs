use crate::spp::analyse::utilities::code_injector::CodeInjector;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::parser::parser::Parser;
use std::default::Default;

pub struct CommonTypes {}

impl CommonTypes {
    pub fn u8() -> TypeAst {
        let code = "std::U8".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn u16() -> TypeAst {
        let code = "std::U16".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn u32() -> TypeAst {
        let code = "std::U32".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn u64() -> TypeAst {
        let code = "std::U64".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn u128() -> TypeAst {
        let code = "std::U128".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn u256() -> TypeAst {
        let code = "std::U256".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn usize() -> TypeAst {
        let code = "std::USize".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i8() -> TypeAst {
        let code = "std::I8".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i16() -> TypeAst {
        let code = "std::I16".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i32() -> TypeAst {
        let code = "std::I32".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i64() -> TypeAst {
        let code = "std::I64".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i128() -> TypeAst {
        let code = "std::I128".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn i256() -> TypeAst {
        let code = "std::I256".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f8() -> TypeAst {
        let code = "std::F8".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f16() -> TypeAst {
        let code = "std::F16".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f32() -> TypeAst {
        let code = "std::F32".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f64() -> TypeAst {
        let code = "std::F64".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f128() -> TypeAst {
        let code = "std::F128".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn f256() -> TypeAst {
        let code = "std::F256".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn big_int() -> TypeAst {
        let code = "std::BigInt".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn big_dec() -> TypeAst {
        let code = "std::BigDec".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn bool() -> TypeAst {
        let code = "std::Bool".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn void() -> TypeAst {
        let code = "std::Void".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn string() -> TypeAst {
        let code = "std::Str".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn copy() -> TypeAst {
        let code = "std::Copy".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn self_() -> TypeAst {
        let code = "Self".to_string();
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn optional(type_: TypeAst, ) -> TypeAst {
        let code = format!("std::Opt[{:?}]", type_);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn future(type_: TypeAst, ) -> TypeAst {
        let code = format!("std::Fut[{:?}]", type_);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn array(type_: TypeAst, size: TokenAst, ) -> TypeAst {
        let code = format!("std::Arr[{:?}, {:?}]", type_, size);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn tuple(types: Vec<TypeAst>, ) -> TypeAst {
        let code = format!(
            "std::Var[{:?}]",
            types
                .iter()
                .map(|t| { format!("{:?}", t) })
                .collect::<Vec<String>>()
                .join(", ")
        );
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn variant(types: Vec<TypeAst>, ) -> TypeAst {
        let code = format!(
            "std::Var[{:?}]",
            types
                .iter()
                .map(|t| { format!("{:?}", t) })
                .collect::<Vec<String>>()
                .join(", ")
        );
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn intersection(types: Vec<TypeAst>, ) -> TypeAst {
        let code = format!(
            "std::Isc[{:?}]",
            types
                .iter()
                .map(|t| { format!("{:?}", t) })
                .collect::<Vec<String>>()
                .join(", ")
        );
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn function_ref(args: TypeAst, return_type: TypeAst, ) -> TypeAst {
        let code = format!("std::FunRef[{:?}, {:?}]", args, return_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn function_mut(args: TypeAst, return_type: TypeAst, ) -> TypeAst {
        let code = format!("std::FunMut[{:?}, {:?}]", args, return_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn function_mov(args: TypeAst, return_type: TypeAst, ) -> TypeAst {
        let code = format!("std::FunMov[{:?}, {:?}]", args, return_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn generator_ref(yield_type: TypeAst, send_type: TypeAst, ) -> TypeAst {
        let code = format!("std::GenRef[{:?}, {:?}]", yield_type, send_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn generator_mut(yield_type: TypeAst, send_type: TypeAst, ) -> TypeAst {
        let code = format!("std::GenMut[{:?}, {:?}]", yield_type, send_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }

    pub fn generator_mov(yield_type: TypeAst, send_type: TypeAst, ) -> TypeAst {
        let code = format!("std::GenMov[{:?}, {:?}]", yield_type, send_type);
        CodeInjector::inject_code(code, Parser::parse_type)
    }
}
