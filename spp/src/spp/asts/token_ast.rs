use crate::spp::asts::ast::Ast;
use crate::spp::lexer::token::TokenType;

#[derive(Clone, Debug)]
pub struct TokenAst {
    pub pos: usize,
    pub token_type: TokenType,
    pub metadata: String,
}

impl TokenAst {
    pub fn new(pos: usize, token_type: TokenType, metadata: String) -> Self {
        Self {
            pos,
            token_type,
            metadata,
        }
    }

    pub fn new_from_pos(pos: usize) -> Self {
        Self {
            pos,
            token_type: TokenType::NoToken,
            metadata: String::new(),
        }
    }
}

impl Default for TokenAst {
    fn default() -> Self {
        Self {
            pos: 0,
            token_type: TokenType::NoToken,
            metadata: String::new(),
        }
    }
}

impl Ast for TokenAst {
    fn get_pos(&self) -> usize {
        self.pos
    }

    fn get_final_pos(&self) -> usize {
        self.pos + 1  // ?
    }
}
