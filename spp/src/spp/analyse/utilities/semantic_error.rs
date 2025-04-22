use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SemanticError {}

impl SemanticError {
    pub fn new_annotation_invalid_application_error(
        annotation: &IdentifierAst,
        applied_to: &impl Ast,
        allow_list: String,
        help: String,
        note: String,
    ) {
        let b0 = Self::add_header(
            1,
            format!("invalid application of annotation `{}`", annotation.value),
        );

        let b1 = Self::add_context_for_error(
            annotation.get_pos(),
            "annotation applied here".to_string(),
        );

        let b2 = Self::add_error(
            applied_to.get_pos(),
            format!("but `{}` can only be applied to {}", annotation.value, allow_list),
        );

        let b3 = Self::add_footer(note, help);
    }
}

impl SemanticError {
    fn add_header(code: u32, msg: String) -> String {
        String::new()
    }
    fn add_error(pos: usize, tag: String) -> String {
        String::new()
    }
    fn add_context_for_error(pos: usize, tag: String) -> String {
        String::new()
    }
    fn add_footer(note: String, help: String) -> String {
        String::new()
    }
}

pub struct SemanticErrorInfo {
    pub pos: usize,
    pub tag: String,
    pub msg: String,
    pub tip: String,
    pub fmt: SemanticErrorFormat,
}

pub enum SemanticErrorFormat {
    NORMAL,
    MINIMAL,
    NONE,
}
