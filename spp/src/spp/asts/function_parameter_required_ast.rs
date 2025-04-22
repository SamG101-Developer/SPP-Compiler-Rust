use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterRequiredAst {
    pos: usize,
    variable: LocalVariableAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl FunctionParameterRequiredAst {
    pub fn new(
        pos: usize,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            variable,
            tok_colon,
            type_,
        }
    }
}

impl Ast for FunctionParameterRequiredAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
