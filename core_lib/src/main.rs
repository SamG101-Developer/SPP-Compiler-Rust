#![feature(trait_alias)]
#![feature(unboxed_closures)]

mod lexer;
mod parser;
mod asts;
mod analyse;

struct DebugShort<'a>(&'a str);

impl std::fmt::Debug for DebugShort<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.0.len() > 0 { &self.0[..10] } else { self.0 };
        write!(f, "{}", s)
    }
}

fn main() {
    let code = std::fs::read_to_string("test_code/2.spp").unwrap();

    let start_time = std::time::Instant::now();
    let tokens = lexer::lexer::Lexer{code}.lex();
    let end_time = std::time::Instant::now();
    println!("Lexing: {} tokens ({}ms)", tokens.len(), (end_time - start_time).as_millis());

    let start_time = std::time::Instant::now();
    let ast = parser::parser::Parser::new(tokens).parse();
    let end_time = std::time::Instant::now();
    println!("Parsing: '{:?}' ({}ms)", DebugShort(&format!("{:?}", ast)), (end_time - start_time).as_millis());
}
