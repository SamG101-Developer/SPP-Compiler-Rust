use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub enum LetStatementAst {
    Initialized {
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    },
    Uninitialized {
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    },
}

impl LetStatementAst {
    pub fn new_initialized(
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
    ) -> Self {
        Self::Initialized {
            pos,
            tok_let,
            assign_to,
            tok_assign,
            value,
        }
    }

    pub fn new_uninitialized(
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self::Uninitialized {
            pos,
            tok_let,
            assign_to,
            tok_colon,
            type_,
        }
    }
}

impl Ast for LetStatementAst {
    fn get_pos(&self) -> usize {
        match self {
            LetStatementAst::Initialized { pos, .. } => *pos,
            LetStatementAst::Uninitialized { pos, .. } => *pos,
        }
    }

    fn get_final_pos(&self) -> usize {
        match self {
            LetStatementAst::Initialized { value, .. } => value.get_final_pos(),
            LetStatementAst::Uninitialized { type_, .. } => type_.get_final_pos(),
        }
    }
}
