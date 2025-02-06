use crate::spp::asts::ast::Ast;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::local_variable_ast::LocalVariableNestedForAttributeBindingAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LocalVariableAttributeBindingAst {
    pub pos: usize,
    pub name: IdentifierAst,
    pub tok_assign: TokenAst,
    pub value: LocalVariableNestedForAttributeBindingAst,
}

impl LocalVariableAttributeBindingAst {
    pub fn new(
        pos: usize,
        name: IdentifierAst,
        tok_assign: TokenAst,
        value: LocalVariableNestedForAttributeBindingAst,
    ) -> Self {
        Self {
            pos,
            name,
            tok_assign,
            value,
        }
    }
}

impl Ast for LocalVariableAttributeBindingAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
