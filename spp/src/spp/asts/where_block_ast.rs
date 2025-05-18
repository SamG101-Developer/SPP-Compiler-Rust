use crate::spp::asts::ast::Ast;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::where_constraints_group_ast::WhereConstraintsGroupAst;

#[derive(Clone, Debug)]
pub struct WhereBlockAst {
    pub tok_where: TokenAst,
    pub constraints: WhereConstraintsGroupAst,
}

impl WhereBlockAst {
    pub fn new(tok_where: TokenAst, constraints: WhereConstraintsGroupAst) -> Self {
        Self { tok_where, constraints }
    }
}

impl Ast for WhereBlockAst {
    fn get_pos(&self) -> usize {
        self.tok_where.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.constraints.get_final_pos()
    }
}
