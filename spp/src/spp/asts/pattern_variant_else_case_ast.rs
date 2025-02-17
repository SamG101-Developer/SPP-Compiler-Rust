use crate::spp::asts::ast::Ast;
use crate::spp::asts::case_expression_ast::CaseExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
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

impl Ast for PatternVariantElseCaseAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.case_expression.get_final_pos()
    }
}
