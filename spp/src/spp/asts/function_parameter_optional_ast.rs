use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct FunctionParameterOptionalAst {
    pub variable: LocalVariableAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    pub tok_assign: TokenAst,
    pub value: ExpressionAst,
}

impl FunctionParameterOptionalAst {
    pub fn new(variable: LocalVariableAst, tok_colon: TokenAst, type_: TypeAst, tok_assign: TokenAst, value: ExpressionAst) -> Self {
        Self { variable, tok_colon, type_, tok_assign, value }
    }
}

impl Ast for FunctionParameterOptionalAst {
    fn get_pos(&self) -> usize {
        self.variable.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
