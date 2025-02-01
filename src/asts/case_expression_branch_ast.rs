use crate::asts::token_ast::TokenAst;

pub struct CaseExpressionBranchAst {
    pub pos: usize,
    pub comp_op: Option<TokenAst>,
    pub patterns: Vec<PatternAst>,
    pub guard: Option<PatternGuardAst>,
    pub body: InnerScopeAst,
}

impl CaseExpressionBranchAst {
    pub fn new(
        pos: usize,
        comp_op: Option<TokenAst>,
        patterns: Vec<PatternAst>,
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
