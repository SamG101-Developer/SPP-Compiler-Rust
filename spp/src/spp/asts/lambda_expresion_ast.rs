use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::lambda_expression_parameter_and_capture_group_ast::LambdaExpressionParameterAndCaptureGroupAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LambdaExpressionAst {
    pub tok_cor: Option<TokenAst>,
    pub pc_group: LambdaExpressionParameterAndCaptureGroupAst,
    pub body: ExpressionAst,
    _ret_type: TypeAst
}

impl LambdaExpressionAst {
    pub fn new(tok_cor: Option<TokenAst>, pc_group: LambdaExpressionParameterAndCaptureGroupAst, body: ExpressionAst, _ret_type: TypeAst) -> Self {
        Self { tok_cor, pc_group, body, _ret_type }
    }
}

impl LambdaExpressionAst {
    pub fn get_pos(&self) -> usize {
        self.tok_cor.as_ref().unwrap().get_pos()
    }

    pub fn get_final_pos(&self) -> usize {
        self.body.get_final_pos()
    }
}
