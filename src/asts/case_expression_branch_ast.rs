use crate::asts::ast::Ast;
use crate::asts::inner_scope_ast::InnerScopeAst;
use crate::asts::pattern_guard_ast::PatternGuardAst;
use crate::asts::pattern_variant_ast::PatternVariantAst;
use crate::asts::token_ast::TokenAst;

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
}

impl Ast for CaseExpressionBranchAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
