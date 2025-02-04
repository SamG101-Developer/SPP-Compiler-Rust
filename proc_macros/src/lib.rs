use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn should_parse_pass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_body = &input.block;
    let fn_signature = &input.sig;

    // Generate a new function that wraps the original logic
    let expanded = quote! {
        #fn_signature {
            let mut parser = Parser::new(Lexer::new(#fn_body.to_string()).lex());
            let result = parser.parse();
            assert!(result.is_ok());
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn should_parse_fail(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_body = &input.block;
    let fn_signature = &input.sig;

    // Generate a new function that wraps the original logic
    let expanded = quote! {
        #fn_signature {
            let mut parser = Parser::new(Lexer::new(#fn_body.to_string()).lex());
            let result = parser.parse();
            assert!(result.is_err());
        }
    };

    TokenStream::from(expanded)
}
