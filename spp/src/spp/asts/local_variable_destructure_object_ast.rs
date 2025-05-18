use crate::spp::asts::ast::Ast;
use crate::spp::asts::local_variable_ast::LocalVariableNestedForDestructureObjectAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LocalVariableDestructureObjectAst {
    pub type_: TypeAst,
    pub tok_parenthesis_l: TokenAst,
    pub elements: Vec<LocalVariableNestedForDestructureObjectAst>,
    pub tok_parenthesis_r: TokenAst,
}

impl LocalVariableDestructureObjectAst {
    pub fn new(type_: TypeAst, tok_parenthesis_l: TokenAst, elements: Vec<LocalVariableNestedForDestructureObjectAst>, tok_parenthesis_r: TokenAst) -> Self {
        Self { type_, tok_parenthesis_l, elements, tok_parenthesis_r }
    }
}

impl Ast for LocalVariableDestructureObjectAst {
    fn get_pos(&self) -> usize {
        self.type_.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.tok_parenthesis_r.get_final_pos()
    }
}
