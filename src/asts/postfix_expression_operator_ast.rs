pub enum PostfixExpressionOperatorAst {
    EarlyReturn(PostfixExpressionOperatorEarlyReturnAst),
    FunctionCall(PostfixExpressionOperatorFunctionCallAst),
    MemberAccess(PostfixExpressionOperatorMemberAccessAst),
    NotKeyword(PostfixExpressionOperatorNotKeywordAst),
    StepKeyword(PostfixExpressionOperatorStepKeywordAst),
}
