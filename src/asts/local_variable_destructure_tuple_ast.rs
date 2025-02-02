use crate::asts::local_variable_ast::LocalVariableNestedForDestructureTupleAst;
use crate::asts::token_ast::TokenAst;

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
