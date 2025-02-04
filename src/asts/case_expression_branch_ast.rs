use crate::asts::ast::Ast;
use crate::asts::expression_ast::ExpressionAst;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::pattern_guard_ast::PatternGuardAst;
use crate::asts::pattern_variant_ast::PatternVariantAst;
use crate::asts::pattern_variant_else_ast::PatternVariantElseAst;
use crate::asts::primary_expression_ast::PrimaryExpressionAst;
use crate::asts::statement_ast::StatementAst;
use crate::asts::token_ast::TokenAst;

#[derive(Clone)]
pub struct CaseExpressionBranchAst {
    pub pos: usize,
    pub comp_op: Option<TokenAst>,
    pub patterns: Vec<PatternVariantAst>,
    pub guard: Option<PatternGuardAst>,
    pub body: InnerScopeAst,
}

impl CaseExpressionBranchAst {
    pub fn new(
        pos: usize,
        comp_op: Option<TokenAst>,
        patterns: Vec<PatternVariantAst>,
        guard: Option<PatternGuardAst>,
        body: InnerScopeAst,
    ) -> Self {
        Self {
            pos,
            comp_op,
            patterns,
            guard,
            body,
        }
    }

    pub fn new_from_else_to_else_case(pos: usize, else_case: PatternVariantAst) -> Self {
        if let PatternVariantAst::ElseCase(else_case) = else_case {
            let else_pattern = PatternVariantElseAst::new(pos, else_case.tok_else);
            let else_body    = InnerScopeAst::new(pos, Default::default(), vec![StatementAst::Expression(ExpressionAst::Primary(PrimaryExpressionAst::Case(else_case.case_expression)))], Default::default());
            let case_branch  = CaseExpressionBranchAst::new(pos, None, vec![PatternVariantAst::Else(else_pattern)], None, else_body);
            case_branch
        } else {
            panic!("Expected PatternVariantAst::ElseCase");
        }
    }
}

impl Ast for CaseExpressionBranchAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
