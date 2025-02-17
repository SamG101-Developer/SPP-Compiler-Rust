use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::generic_parameter_constraints_ast::GenericParameterConstraintsAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub enum GenericParameterAst {
    CompRequired {
        pos: usize,
        tok_comp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    },

    CompOptional {
        pos: usize,
        tok_cmp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        default: ExpressionAst,
    },

    CompVariadic {
        pos: usize,
        tok_cmp: TokenAst,
        tok_variadic: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    },

    TypeRequired {
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    },

    TypeOptional {
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
        tok_assign: TokenAst,
        default: TypeAst,
    },

    TypeVariadic {
        pos: usize,
        tok_variadic: TokenAst,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    },
}

impl GenericParameterAst {
    pub fn new_comp_required(
        pos: usize,
        tok_comp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self::CompRequired {
            pos,
            tok_comp,
            name,
            tok_colon,
            type_,
        }
    }

    pub fn new_comp_optional(
        pos: usize,
        tok_cmp: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        default: ExpressionAst,
    ) -> Self {
        Self::CompOptional {
            pos,
            tok_cmp,
            name,
            tok_colon,
            type_,
            tok_assign,
            default,
        }
    }

    pub fn new_comp_variadic(
        pos: usize,
        tok_cmp: TokenAst,
        tok_variadic: TokenAst,
        name: TypeAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self::CompVariadic {
            pos,
            tok_cmp,
            tok_variadic,
            name,
            tok_colon,
            type_,
        }
    }

    pub fn new_type_required(
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    ) -> Self {
        Self::TypeRequired {
            pos,
            name,
            constraints,
        }
    }

    pub fn new_type_optional(
        pos: usize,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
        tok_assign: TokenAst,
        default: TypeAst,
    ) -> Self {
        Self::TypeOptional {
            pos,
            name,
            constraints,
            tok_assign,
            default,
        }
    }

    pub fn new_type_variadic(
        pos: usize,
        tok_variadic: TokenAst,
        name: TypeAst,
        constraints: Option<GenericParameterConstraintsAst>,
    ) -> Self {
        Self::TypeVariadic {
            pos,
            tok_variadic,
            name,
            constraints,
        }
    }
}

impl Ast for GenericParameterAst {
    fn get_pos(&self) -> usize {
        match self {
            GenericParameterAst::CompRequired { pos, .. } => *pos,
            GenericParameterAst::CompOptional { pos, .. } => *pos,
            GenericParameterAst::CompVariadic { pos, .. } => *pos,
            GenericParameterAst::TypeRequired { pos, .. } => *pos,
            GenericParameterAst::TypeOptional { pos, .. } => *pos,
            GenericParameterAst::TypeVariadic { pos, .. } => *pos,
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            GenericParameterAst::CompRequired { type_, .. } => type_.get_final_pos(),
            GenericParameterAst::CompOptional { default, .. } => default.get_final_pos(),
            GenericParameterAst::CompVariadic { type_, .. } => type_.get_final_pos(),
            GenericParameterAst::TypeRequired { name, .. } => name.get_final_pos(),
            GenericParameterAst::TypeOptional { default, .. } => default.get_final_pos(),
            GenericParameterAst::TypeVariadic { name, .. } => name.get_final_pos(),
        }
    }
}
