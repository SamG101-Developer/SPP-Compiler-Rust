use crate::spp::asts::ast::Ast;
use crate::spp::asts::expression_ast::ExpressionAst;
use crate::spp::asts::local_variable_ast::LocalVariableAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;

#[derive(Clone, Debug)]
pub struct LetStatementInitializedAst {
    tok_let: TokenAst,
    assign_to: LocalVariableAst,
    explicit_type: Option<TypeAst>,
    tok_assign: TokenAst,
    value: ExpressionAst,
}

impl LetStatementInitializedAst {
    pub fn new(tok_let: TokenAst, assign_to: LocalVariableAst, explicit_type: Option<TypeAst>, tok_assign: TokenAst, value: ExpressionAst) -> Self {
        Self { tok_let, assign_to, explicit_type, tok_assign, value}
    }
}

impl Ast for LetStatementInitializedAst {
    fn get_pos(&self) -> usize {
        self.tok_let.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.value.get_final_pos()
    }
}
