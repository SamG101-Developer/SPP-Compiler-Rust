use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterVariadicAst {
    pos: usize,
    tok_variadic: TokenAst,
    variable: LocalVariableAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl FunctionParameterVariadicAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        variable: LocalVariableAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            variable,
            tok_colon,
            type_,
        }
    }
}

impl Ast for FunctionParameterVariadicAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
