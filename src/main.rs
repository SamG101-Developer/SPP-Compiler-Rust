#![feature(trait_alias)]
#![feature(unboxed_closures)]

mod lexer;
mod parser;
mod asts;
mod analyse;

fn main() {
    let code = std::fs::read_to_string("test_code/2.spp").unwrap();

    let start_time = std::time::Instant::now();
    let tokens = lexer::lexer::Lexer{code}.lex();
    let end_time = std::time::Instant::now();
    println!("Lexing: {} ({}ms)", tokens.len(), (end_time - start_time).as_millis());

    let start_time = std::time::Instant::now();
    let ast = parser::parser::Parser::new(tokens).parse();
    let end_time = std::time::Instant::now();
    println!("Parsing: Done ({}ms)", (end_time - start_time).as_millis());
}
