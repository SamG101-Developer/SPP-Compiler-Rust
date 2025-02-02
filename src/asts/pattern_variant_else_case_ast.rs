use crate::asts::case_expression_ast::CaseExpressionAst;
use crate::asts::token_ast::TokenAst;

pub struct PatternVariantElseCaseAst {
    pub pos: usize,
    pub tok_else: TokenAst,
    pub case_expression: CaseExpressionAst,
}

impl PatternVariantElseCaseAst {
    pub fn new(pos: usize, tok_else: TokenAst, case_expression: CaseExpressionAst) -> Self {
        Self {
            pos,
            tok_else,
            case_expression,
        }
    }
}
