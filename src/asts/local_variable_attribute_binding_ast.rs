use crate::asts::ast::Ast;
use crate::asts::identifier_ast::IdentifierAst;
use crate::asts::local_variable_ast::LocalVariableNestedForAttributeBindingAst;
use crate::asts::token_ast::TokenAst;

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
}
