use crate::lexer::token::TokenType;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    pub pos: usize,
    pub expected_tokens: HashSet<TokenType>,
    pub message: String,
}

impl SyntaxError {
    pub fn new(pos: usize, message: String) -> Self {
        Self {
            pos,
            expected_tokens: HashSet::new(),
            message,
        }
    }
}
