use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_single_identifier_ast::LocalVariableSingleIdentifierAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

    fn get_final_pos(&self) -> usize {
        if let Some(binding) = &self.binding {
            binding.get_final_pos()
        } else {
            self.tok_variadic.get_final_pos()
        }
    }
}
