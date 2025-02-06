use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct WithExpressionAliasAst {
    pub pos: usize,
    pub tok_as: TokenAst,
    pub variable: LocalVariableAst,
}

impl WithExpressionAliasAst {
    pub fn new(pos: usize, tok_as: TokenAst, variable: LocalVariableAst) -> WithExpressionAliasAst {
        WithExpressionAliasAst {
            pos,
            tok_as,
            variable,
        }
    }
}

impl Ast for WithExpressionAliasAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
