use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct UnaryExpressionOperatorAsyncAst {
    pub pos: usize,
    pub tok_async: TokenAst,
}

impl UnaryExpressionOperatorAsyncAst {
    pub fn new(pos: usize, tok_async: TokenAst) -> Self {
        Self { pos, tok_async }
    }
}

impl Ast for UnaryExpressionOperatorAsyncAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_async.get_final_pos()
    }
}
