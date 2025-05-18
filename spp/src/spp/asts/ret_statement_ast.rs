use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct RetStatementAst {
    pub tok_ret: TokenAst,
    pub expr: Option<ExpressionAst>,
    _func_ret_type: Option<TypeAst>,
}

impl RetStatementAst {
    pub fn new(tok_ret: TokenAst, expr: Option<ExpressionAst>) -> Self {
        Self { tok_ret, expr, _func_ret_type: None, }
    }
}

impl Ast for RetStatementAst {
    fn get_pos(&self) -> usize {
        self.tok_ret.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.expr.as_ref().map_or(self.tok_ret.get_final_pos(), |a| a.get_final_pos())
    }
}
