use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LetStatementUninitializedAst {
    pos: usize,
    tok_let: TokenAst,
    assign_to: LocalVariableAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl LetStatementUninitializedAst {
    pub fn new(
        pos: usize,
        tok_let: TokenAst,
        assign_to: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_let,
            assign_to,
            tok_colon,
            type_,
        }
    }
}

impl Ast for LetStatementUninitializedAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
