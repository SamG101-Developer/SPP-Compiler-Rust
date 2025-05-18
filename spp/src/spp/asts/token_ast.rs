use crate::spp::asts::ast::Ast;
use crate::spp::lexer::token::TokenAstTokenType;

#[derive(Clone, Debug)]
pub struct TokenAst {
    pos: usize,
    pub token_type: TokenAstTokenType,
    pub metadata: String,
}

impl TokenAst {
    pub fn new(pos: usize, token_type: TokenAstTokenType, metadata: String) -> Self {
        Self { pos, token_type, metadata }
    }
    
    pub fn new_from_tok(token_type: TokenAstTokenType) -> Self {
        Self { pos: 0, token_type, metadata: String::new() }
    }

    pub fn new_from_pos(pos: usize) -> Self {
        Self { pos, token_type: TokenAstTokenType::NoToken, metadata: String::new() }
    }
}

impl Default for TokenAst {
    fn default() -> Self {
        Self {
            pos: 0,
            token_type: TokenAstTokenType::NoToken,
            metadata: String::new(),
        }
    }
}

impl Ast for TokenAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.pos + self.metadata.len()
    }
}
