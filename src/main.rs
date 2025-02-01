mod lexer;
mod parser;
mod asts;

fn main() {
    let code = std::fs::read_to_string("test_code/1.spp").unwrap();

    let start_time = std::time::Instant::now();
    let tokens = lexer::lexer::Lexer{code}.lex();
    let end_time = std::time::Instant::now();


    println!("{} ({}ms)", tokens.len(), (end_time - start_time).as_millis());
}
