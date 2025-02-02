use crate::asts::object_initializer_argument_ast::ObjectInitializerArgumentAst;
use crate::asts::token_ast::TokenAst;

pub struct ObjectInitializerArgumentGroupAst {
    pub pos: usize,
    pub tok_parenthesis_l: TokenAst,
    pub args: Vec<ObjectInitializerArgumentAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl ObjectInitializerArgumentGroupAst {
    pub fn new(
        pos: usize,
        tok_parenthesis_l: TokenAst,
        args: Vec<ObjectInitializerArgumentAst>,
        tok_parenthesis_r: TokenAst,
    ) -> Self {
        Self {
            pos,
            tok_parenthesis_l,
            args,
            tok_parenthesis_r,
        }
    }
}
