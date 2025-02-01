use crate::asts::expression_ast::ExpressionAst;
use crate::asts::token_ast::TokenAst;

pub struct CaseExpressionAst {
    pub pos: usize,
    pub tok_case: TokenAst,
    pub condition: ExpressionAst,
    pub tok_of: TokenAst,
    pub branches: Vec<CaseExpressionBranchAst>,
}

impl CaseExpressionAst {
    pub fn new(
        pos: usize,
        tok_case: TokenAst,
        condition: ExpressionAst,
        tok_of: TokenAst,
        branches: Vec<CaseExpressionBranchAst>,
    ) -> Self {
        Self {
            pos,
            tok_case,
            condition,
            tok_of,
            branches,
        }
    }
}
