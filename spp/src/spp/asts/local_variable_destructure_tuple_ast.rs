use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableNestedForDestructureTupleAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug)]
pub struct LocalVariableDestructureTupleAst {
    pub pos: usize,
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<LocalVariableNestedForDestructureTupleAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl LocalVariableDestructureTupleAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        elements: Vec<LocalVariableNestedForDestructureTupleAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_l,
            elements,
            tok_parenthesis_r,
        }
    }
}

impl Ast for LocalVariableDestructureTupleAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
