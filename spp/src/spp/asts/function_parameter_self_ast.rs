use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::lexer::token::TokenAstTokenType;

#[derive(Clone, Debug)]
pub struct FunctionParameterSelfAst {
    pub tok_mut: Option<TokenAst>,
    pub convention: Option<ConventionAst>,
    pub name: IdentifierAst,
    pub tok_colon: TokenAst,
    pub type_: TypeAst,
    
    _is_arbitrary: bool,
    _true_self_type: Option<TypeAst>,
}

impl FunctionParameterSelfAst {
    pub fn new(tok_mut: Option<TokenAst>, convention: Option<ConventionAst>, name: IdentifierAst, ) -> Self {
        Self { tok_mut, convention, name, tok_colon: TokenAst::new(0, TokenAstTokenType::TkColon, String::new()), type_: CommonTypes::self_(), _is_arbitrary: false, _true_self_type: None, }
    }
    
    pub fn new_with_type(tok_mut: Option<TokenAst>, convention: Option<ConventionAst>, name: IdentifierAst, tok_colon: TokenAst, type_: TypeAst) -> Self {
        Self { tok_mut, convention, name, tok_colon, type_, _is_arbitrary: true, _true_self_type: None, }
    }
}

impl FunctionParameterSelfAst {
    pub fn set_true_self_type_from_context(&mut self, true_self_type: TypeAst) {
        self._true_self_type = Some(true_self_type);
    }
}

impl Ast for FunctionParameterSelfAst {
    fn get_pos(&self) -> usize {
        self.name.get_pos()
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
