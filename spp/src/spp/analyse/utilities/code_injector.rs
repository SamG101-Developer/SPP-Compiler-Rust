use crate::spp::lexer::lexer::Lexer;
use crate::spp::parser::parser::{Parser, ParserResult};

pub struct CodeInjector {}

impl CodeInjector {
    pub fn inject_code<T>(code: String, mut parsing_function: impl FnMut(&mut Parser) -> ParserResult<T>) -> T {
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer.lex());
        parsing_function(&mut parser).expect("Auto generated code is invalid")
    }
}
