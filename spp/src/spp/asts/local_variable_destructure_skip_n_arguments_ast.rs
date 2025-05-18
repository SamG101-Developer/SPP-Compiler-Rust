use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LocalVariableDestructureSkipNArgumentsAst {
    tok_variadic: TokenAst,
    binding: Option<LocalVariableSingleIdentifierAst>,
}

impl LocalVariableDestructureSkipNArgumentsAst {
    pub fn new(tok_variadic: TokenAst, binding: Option<LocalVariableSingleIdentifierAst>) -> Self {
        Self { tok_variadic, binding }
    }
}

impl Ast for LocalVariableDestructureSkipNArgumentsAst {
    fn get_pos(&self) -> usize {
        self.tok_variadic.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.binding.as_ref().map_or(self.tok_variadic.get_final_pos(), |b| b.get_pos())
    }
}
