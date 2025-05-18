use crate::spp::asts::ast::Ast;
use crate::spp::asts::lambda_expression_capture_item_ast::LambdaExpressionCaptureItemAst;
use crate::spp::asts::lambda_expression_parameter_ast::LambdaExpressionParameterAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LambdaExpressionParameterAndCaptureGroupAst {
    pub tok_l: TokenAst,
    pub params: Vec<LambdaExpressionParameterAst>,
    pub captures: Vec<LambdaExpressionCaptureItemAst>,
    pub tok_r: TokenAst,
}

impl LambdaExpressionParameterAndCaptureGroupAst {
    pub fn new(tok_l: TokenAst, params: Vec<LambdaExpressionParameterAst>, captures: Vec<LambdaExpressionCaptureItemAst>, tok_r: TokenAst) -> Self {
        Self { tok_l, params, captures, tok_r }
    }
}

impl LambdaExpressionParameterAndCaptureGroupAst {
    pub fn get_pos(&self) -> usize {
        self.tok_l.get_pos()
    }

    pub fn get_final_pos(&self) -> usize {
        self.tok_r.get_final_pos()
    }
}
