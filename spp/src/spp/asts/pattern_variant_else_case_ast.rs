use crate::spp::asts::ast::Ast;
use crate::spp::asts::case_expression_ast::CaseExpressionAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct PatternVariantElseCaseAst {
    pub tok_else: TokenAst,
    pub case_expression: CaseExpressionAst,
}

impl PatternVariantElseCaseAst {
    pub fn new(tok_else: TokenAst, case_expression: CaseExpressionAst) -> Self {
        Self { tok_else, case_expression }
    }
}

impl Ast for PatternVariantElseCaseAst {
    fn get_pos(&self) -> usize {
        self.tok_else.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.case_expression.get_final_pos()
    }
}
