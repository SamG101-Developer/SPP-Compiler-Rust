use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub enum LiteralAst {
    Boolean {
        pos: usize,
        value: TokenAst,
    },
    Integer {
        pos: usize,
        tok_sign: Option<TokenAst>,
        integer_value: TokenAst,
        type_: Option<TypeAst>,
    },
    Float {
        pos: usize,
        tok_sign: Option<TokenAst>,
        integer_value: TokenAst,
        tok_dot: TokenAst,
        float_value: TokenAst,
        type_: Option<TypeAst>,
    },
    String {
        pos: usize,
        value: TokenAst,
    },
    Array0 {
        pos: usize,
        tok_bracket_l: TokenAst,
        elem_type: TypeAst,
        tok_comma: TokenAst,
        size: TokenAst,
        tok_bracket_r: TokenAst,
    },
    ArrayN {
        pos: usize,
        tok_bracket_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_bracket_r: TokenAst,
    },
    Tuple {
        pos: usize,
        tok_parenthesis_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_parenthesis_r: TokenAst,
    },
}

impl LiteralAst {
    pub fn new_boolean(pos: usize, value: TokenAst) -> Self {
        LiteralAst::Boolean { pos, value }
    }

    pub fn new_integer(
        pos: usize,
        tok_sign: Option<TokenAst>,
        integer_value: TokenAst,
        type_: Option<TypeAst>,
    ) -> Self {
        LiteralAst::Integer {
            pos,
            tok_sign,
            integer_value,
            type_,
        }
    }

    pub fn new_float(
        pos: usize,
        tok_sign: Option<TokenAst>,
        integer_value: TokenAst,
        tok_dot: TokenAst,
        float_value: TokenAst,
        type_: Option<TypeAst>,
    ) -> Self {
        LiteralAst::Float {
            pos,
            tok_sign,
            integer_value,
            tok_dot,
            float_value,
            type_,
        }
    }

    pub fn new_string(pos: usize, value: TokenAst) -> Self {
        LiteralAst::String { pos, value }
    }

    pub fn new_array_0(
        pos: usize,
        tok_bracket_l: TokenAst,
        elem_type: TypeAst,
        tok_comma: TokenAst,
        size: TokenAst,
        tok_bracket_r: TokenAst,
    ) -> Self {
        LiteralAst::Array0 {
            pos,
            tok_bracket_l,
            elem_type,
            tok_comma,
            size,
            tok_bracket_r,
        }
    }

    pub fn new_array_n(
        pos: usize,
        tok_bracket_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_bracket_r: TokenAst,
    ) -> Self {
        LiteralAst::ArrayN {
            pos,
            tok_bracket_l,
            elements,
            tok_bracket_r,
        }
    }

    pub fn new_tuple(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        elements: Vec<ExpressionAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        LiteralAst::Tuple {
            pos,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for LiteralAst {
    fn get_pos(&self) -> usize {
        match self {
            LiteralAst::Boolean { pos, .. } => *pos,
            LiteralAst::Integer { pos, .. } => *pos,
            LiteralAst::Float { pos, .. } => *pos,
            LiteralAst::String { pos, .. } => *pos,
            LiteralAst::Array0 { pos, .. } => *pos,
            LiteralAst::ArrayN { pos, .. } => *pos,
            LiteralAst::Tuple { pos, .. } => *pos,
        }
    }
}
