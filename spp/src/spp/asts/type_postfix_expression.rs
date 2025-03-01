use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_postfix_expression_operator_ast::TypePostfixExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct TypePostfixExpressionAst {
    pub pos: usize,
    pub lhs: Box<TypeAst>,
    pub op: TypePostfixExpressionOperatorAst,
}

impl TypePostfixExpressionAst {
    pub fn new(pos: usize, lhs: Box<TypeAst>, op: TypePostfixExpressionOperatorAst) -> Self {
        Self { pos, lhs, op }
    }
}

impl Ast for TypePostfixExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.op.get_final_pos()
    }
}
