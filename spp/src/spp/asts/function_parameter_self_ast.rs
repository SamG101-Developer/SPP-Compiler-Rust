use crate::spp::analyse::utilities::common_types::CommonTypes;
use crate::spp::asts::ast::Ast;
use crate::spp::asts::convention_ast::ConventionAst;
use crate::spp::asts::identifier_ast::IdentifierAst;
use crate::spp::asts::token_ast::TokenAst;
use crate::spp::asts::type_ast::TypeAst;
use crate::spp::lexer::token::TokenAstTokenType;

#[derive(Clone, Debug)]
pub struct FunctionParameterSelfAst {
    pos: usize,
    tok_mut: Option<TokenAst>,
    convention: Option<ConventionAst>,
    name: IdentifierAst,
    tok_colon: TokenAst,
    type_: TypeAst,
}

impl FunctionParameterSelfAst {
    pub fn new(
        pos: usize,
        tok_mut: Option<TokenAst>,
        convention: Option<ConventionAst>,
        name: IdentifierAst,
    ) -> Self {
        Self {
            pos,
            tok_mut,
            convention,
            name,
            tok_colon: TokenAst::new(pos, TokenAstTokenType::TkColon, String::new()),
            type_: CommonTypes::self_(pos),
        }
    }
    
    pub fn new_with_type(
        pos: usize,
        tok_mut: Option<TokenAst>,
        convention: Option<ConventionAst>,
        name: IdentifierAst,
        tok_colon: TokenAst,
        type_: TypeAst,
    ) -> Self {
        Self {
            pos,
            tok_mut,
            convention,
            name,
            tok_colon,
            type_,
        }
    }
}

impl Ast for FunctionParameterSelfAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.type_.get_final_pos()
    }
}
