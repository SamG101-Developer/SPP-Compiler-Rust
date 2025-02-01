extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn parser_rule(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_signature = &input.sig;

    // Generate a new function that wraps the original logic
    let expanded = quote! {
        #fn_signature {
            ParserRuleHandler::new(|| { #fn_body })
        }
    };

    TokenStream::from(expanded)
}
