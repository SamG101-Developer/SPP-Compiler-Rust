use crate::asts::ast::Ast;
use crate::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct LocalVariableDestructureSkipNArgumentsAst {
    pos: usize,
    tok_variadic: TokenAst,
    binding: Option<LocalVariableSingleIdentifierAst>,
}

impl LocalVariableDestructureSkipNArgumentsAst {
    pub fn new(
        pos: usize,
        tok_variadic: TokenAst,
        binding: Option<LocalVariableSingleIdentifierAst>,
    ) -> Self {
        Self {
            pos,
            tok_variadic,
            binding,
        }
    }
}

impl Ast for LocalVariableDestructureSkipNArgumentsAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
