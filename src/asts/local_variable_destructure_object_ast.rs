use crate::asts::ast::Ast;
use crate::asts::local_variable_ast::LocalVariableNestedForDestructureObjectAst;
use crate::asts::token_ast::TokenAst;
use crate::asts::type_ast::TypeAst;

#[derive(Clone)]
pub struct LocalVariableDestructureObjectAst {
    pub pos: usize,
    pub type_: TypeAst,
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<LocalVariableNestedForDestructureObjectAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl LocalVariableDestructureObjectAst {
    pub fn new(
        pos: usize,
        type_: TypeAst,
        tok_parenthesis_l: TokenAst,
        elements: Vec<LocalVariableNestedForDestructureObjectAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            type_,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for LocalVariableDestructureObjectAst {
    fn get_pos(&self) -> usize {
        self.pos
    }
}
