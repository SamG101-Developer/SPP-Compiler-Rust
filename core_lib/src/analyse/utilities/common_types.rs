use std::default::Default;
use crate::asts::generic_argument_ast::GenericArgumentAst;
use crate::asts::generic_argument_group_ast::GenericArgumentGroupAst;
use crate::asts::generic_identifier_ast::GenericIdentifierAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::type_ast::TypeAst;

pub struct CommonTypes { }

impl CommonTypes {
    pub fn optional(type_: TypeAst, pos: usize) -> TypeAst {
        let generic = GenericArgumentAst::TypeUnnamed{pos, type_};
        let generic_group = GenericArgumentGroupAst{pos, args: vec![generic], ..Default::default()};
        let std_namespace = IdentifierAst{pos, value: "std".to_string()};
        let optional_type = GenericIdentifierAst{pos, value: "Opt".to_string(), generic_args_group: Some(generic_group)};
        TypeAst{pos, namespace: vec![std_namespace], types: vec![optional_type]}
    }

    pub fn self_(pos: usize) -> TypeAst {
        let self_type = GenericIdentifierAst{pos, value: "Self".to_string(), generic_args_group: None};
        TypeAst{pos, namespace: vec![], types: vec![self_type]}
    }

    pub fn tuple(types: Vec<TypeAst>, pos: usize) -> TypeAst {
        let generics = types.into_iter().map(|type_| GenericArgumentAst::TypeUnnamed{pos, type_}).collect::<Vec<_>>();
        let generic_group = GenericArgumentGroupAst{pos, args: generics, ..Default::default()};
        let std_namespace = IdentifierAst{pos, value: "std".to_string()};
        let tuple_type = GenericIdentifierAst{pos, value: "Tup".to_string(), generic_args_group: Some(generic_group)};
        TypeAst{pos, namespace: vec![std_namespace], types: vec![tuple_type]}
    }

    pub fn variant(types: Vec<TypeAst>, pos: usize) -> TypeAst {
        let generics = types.into_iter().map(|t| GenericArgumentAst::TypeUnnamed{pos, type_: t}).collect::<Vec<_>>();
        let generic_group = GenericArgumentGroupAst{pos, args: generics, ..Default::default()};
        let std_namespace = IdentifierAst{pos, value: "std".to_string()};
        let tuple_type = GenericIdentifierAst{pos, value: "Var".to_string(), generic_args_group: Some(generic_group)};
        TypeAst{pos, namespace: vec![std_namespace], types: vec![tuple_type]}
    }
}