use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn should_parse_pass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_body = &input.block;
    let fn_signature = &input.sig;

    // Generate a new function that wraps the original logic
    let expanded = quote! {
        #fn_signature {
            let tokens = Lexer::new(#fn_body.to_string()).lex();
            let error_formatter = ErrorFormatter::new("<test>".to_string(), tokens.clone());
            let ast = Parser::new(tokens).parse();
            if let Err(error) = ast {
                let error_string = error_formatter.error_raw_pos(error.get_pos(), 1, error.get_msg(), "Syntax Error".to_string());
                println!("{}", error_string);
                panic!();
            }
            else {
                println!("Parsed successful");
                println!("{:?}", ast.unwrap());
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(SemanticError)]
pub fn derive_semantic_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl crate::spp::analyse::utilities::semantic_error::SemanticError for #name {}
    };
    TokenStream::from(expanded)
}
