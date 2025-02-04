use crate::asts::ast::Ast;
use crate::asts::case_expression_branch_ast::CaseExpressionBranchAst;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::literal_ast::LiteralAst;
use crate::asts::pattern_variant_ast::PatternVariantAst;
use crate::asts::pattern_variant_expression_ast::PatternVariantExpressionAst;
use crate::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct CaseExpressionAst {
    pub pos: usize,
    pub tok_case: TokenAst,
    pub condition: Box<ExpressionAst>,
    pub tok_of: TokenAst,
    pub branches: Vec<CaseExpressionBranchAst>,
}

impl CaseExpressionAst {
    pub fn new(
        pos: usize,
        tok_case: TokenAst,
        condition: Box<ExpressionAst>,
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

    pub fn new_from_simple(pos: usize, tok_case: TokenAst, condition: Box<ExpressionAst>, inner_scope: InnerScopeAst, mut branches: Vec<CaseExpressionBranchAst>) -> Self {
        let first_pattern = PatternVariantAst::Expression(PatternVariantExpressionAst::new(pos, ExpressionAst::Primary(PrimaryExpressionAst::Literal(LiteralAst::Boolean { pos, value: TokenAst::new_from_pos(pos) }))));
        let first_branch  = CaseExpressionBranchAst::new(pos, Some(TokenAst::new_from_pos(pos)), vec![first_pattern], None, inner_scope);
        branches.insert(0, first_branch);
        Self {
            pos,
            tok_case,
            condition,
            tok_of: TokenAst::new_from_pos(pos),
            branches,
        }
    }
}

impl Ast for CaseExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
