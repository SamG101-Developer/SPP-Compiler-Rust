use crate::spp::asts::ast::Ast;
use crate::spp::asts::object_initializer_argument_ast::ObjectInitializerArgumentAst;
use crate::spp::asts::token_ast::TokenAst;

#[derive(Clone, Debug, Default)]
pub struct ObjectInitializerArgumentGroupAst {
    pub tok_parenthesis_l: TokenAst,
    pub args: Vec<ObjectInitializerArgumentAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl ObjectInitializerArgumentGroupAst {
    pub fn new(tok_parenthesis_l: TokenAst, args: Vec<ObjectInitializerArgumentAst>, tok_parenthesis_r: TokenAst) -> Self {
        Self { tok_parenthesis_l, args, tok_parenthesis_r }
    }
}

impl Ast for ObjectInitializerArgumentGroupAst {
    fn get_pos(&self) -> usize {
        self.tok_parenthesis_l.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
