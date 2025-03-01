use crate::spp::asts::ast::Ast;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::asts::type_unary_expression_operator_ast::TypeUnaryExpressionOperatorAst;

#[derive(Clone, Debug)]
pub struct TypeUnaryExpressionAst {
    pub pos: usize,
    pub op: TypeUnaryExpressionOperatorAst,
    pub rhs: Box<TypeAst>,
}

impl TypeUnaryExpressionAst {
    pub fn new(pos: usize, op: TypeUnaryExpressionOperatorAst, rhs: Box<TypeAst>) -> Self {
        Self { pos, op, rhs }
    }
}

impl Ast for TypeUnaryExpressionAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.rhs.get_final_pos()
    }
}
