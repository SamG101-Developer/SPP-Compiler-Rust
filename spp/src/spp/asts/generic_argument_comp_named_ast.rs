use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericArgumentCompNamedAst {
    name: TypeAst,
    tok_assign: TokenAst,
    value: ExpressionAst,
}

impl GenericArgumentCompNamedAst {
    pub fn new(name: TypeAst, tok_assign: TokenAst, value: ExpressionAst) -> Self {
        Self { name, tok_assign, value }
    }
}

impl Ast for GenericArgumentCompNamedAst {
    fn get_pos(&self) -> usize {
        self.name.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
