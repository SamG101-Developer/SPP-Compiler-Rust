use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterVariadicAst {
    pub tok_variadic: TokenAst,
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
}

impl FunctionParameterVariadicAst {
    pub fn new(tok_variadic: TokenAst, variable: LocalVariableAst, tok_colon: TokenAst, type_: TypeAst) -> Self {
        Self { tok_variadic, variable, tok_colon, type_ }
    }
}

impl Ast for FunctionParameterVariadicAst {
    fn get_pos(&self) -> usize {
        self.tok_variadic.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
