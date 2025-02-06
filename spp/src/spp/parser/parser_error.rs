use crate::spp::lexer::token::TokenType;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pos: usize,
    expected_tokens: HashSet<TokenType>,
    message: String,
}

impl SyntaxError {
    pub fn new(pos: usize, message: String) -> Self {
        Self {
            pos,
            expected_tokens: HashSet::new(),
            message,
        }
    }

    pub fn add_expected_token(&mut self, token: TokenType) {
        if self.expected_tokens.insert(token) {
            if let Some(pos) = self.message.find("£") {
                self.message.insert_str(pos, &("'".to_string() + token.to_string().as_str() + "', "));
            }
        }
    }

    pub fn reset(&mut self, pos: usize, message: String) {
        self.pos = pos;
        self.expected_tokens.clear();
        self.message = message;
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn get_msg(&self) -> String {
        self.message.clone()
    }
}
