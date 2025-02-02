use crate::analyse::utilities::common_types::CommonTypes;
use crate::asts::ast::Ast;
use crate::asts::convention_ast::ConventionAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::local_variable_ast::LocalVariableAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

pub enum FunctionParameterAst {
    Self_ {
        pos: usize,
        tok_mut: Option<TokenAst>,
        convention: ConventionAst,
        name: IdentifierAst,
        _type: TypeAst,
    },

    Required {
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    },

    Optional {
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    },

    Variadic {
        pos: usize,
        tok_variadic: TokenAst,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    },
}

impl FunctionParameterAst {
    pub fn new_self(
        pos: usize,
        tok_mut: Option<TokenAst>,
        convention: ConventionAst,
        name: IdentifierAst,
    ) -> Self {
        Self::Self_ {
            pos,
            tok_mut,
            convention,
            name,
            _type: CommonTypes::self_(pos),
        }
    }

    pub fn new_required(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    ) -> Self {
        Self::Required {
            pos,
            variable,
            tok_colon,
            convention,
            type_,
        }
    }

    pub fn new_optional(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    ) -> Self {
        Self::Optional {
            pos,
            variable,
            tok_colon,
            convention,
            type_,
            tok_assign,
            value,
        }
    }

    pub fn new_variadic(
        pos: usize,
        tok_variadic: TokenAst,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        convention: ConventionAst,
        type_: TypeAst,
    ) -> Self {
        Self::Variadic {
            pos,
            tok_variadic,
            variable,
            tok_colon,
            convention,
            type_,
        }
    }
}

impl Ast for FunctionParameterAst {
    fn get_pos(&self) -> usize {
        match self {
            FunctionParameterAst::Self_ { pos, .. } => *pos,
            FunctionParameterAst::Required { pos, .. } => *pos,
            FunctionParameterAst::Optional { pos, .. } => *pos,
            FunctionParameterAst::Variadic { pos, .. } => *pos,
        }
    }
}
