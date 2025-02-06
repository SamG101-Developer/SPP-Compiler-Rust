use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

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

    fn get_final_pos(&self) -> usize {
        if let Some(expr) = &self.expr {
            expr.get_final_pos()
        } else {
            self.tok_ret.get_final_pos()
        }
    }
}
