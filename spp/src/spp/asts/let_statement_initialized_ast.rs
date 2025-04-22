use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LetStatementInitializedAst {
    pos: usize,
    tok_let: TokenAst,
    assign_to: LocalVariableAst,
    tok_assign: TokenAst,
    value: ExpressionAst,
    explicit_type: Option<TypeAst>,
}

impl LetStatementInitializedAst {
    pub fn new(
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_assign: TokenAst,
        value: ExpressionAst,
        explicit_type: Option<TypeAst>,
    ) -> Self {
        Self {
            pos,
            tok_let,
            assign_to,
            tok_assign,
            value,
            explicit_type,
        }
    }
}

impl Ast for LetStatementInitializedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        if let Some(explicit_type) = &self.explicit_type {
            explicit_type.get_final_pos()
        } else {
            self.value.get_final_pos()
        }
    }
}
