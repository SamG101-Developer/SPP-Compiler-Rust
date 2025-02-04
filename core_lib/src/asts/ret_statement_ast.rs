use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct RetStatementAst {
    pub pos: usize,
    pub tok_ret: TokenAst,
    pub expr: Option<ExpressionAst>,
    _func_ret_type: Option<TypeAst>,
}

impl RetStatementAst {
    pub fn new(pos: usize, tok_ret: TokenAst, expr: Option<ExpressionAst>) -> Self {
        Self {
            pos,
            tok_ret,
            expr,
            _func_ret_type: None,
        }
    }
}

impl Ast for RetStatementAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
