use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct GenericParameterCompOptionalAst {
    tok_cmp: TokenAst,
    name: TypeAst,
    tok_colon: TokenAst,
    type_: TypeAst,
    tok_assign: TokenAst,
    default: ExpressionAst,
}

impl GenericParameterCompOptionalAst {
    pub fn new(tok_cmp: TokenAst, name: TypeAst, tok_colon: TokenAst, type_: TypeAst, tok_assign: TokenAst, default: ExpressionAst) -> Self {
        Self { tok_cmp, name, tok_colon, type_, tok_assign, default }
    }
}

impl Ast for GenericParameterCompOptionalAst {
    fn get_pos(&self) -> usize {
        self.tok_cmp.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.default.get_final_pos()
    }
}
