extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn parser_rule(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_body = &input.block;
    let fn_signature = &input.sig;

    // Generate a new function that wraps the original logic
    let expanded = quote! {
        #fn_signature {
            SingleParserRuleHandler{rule: Box::new(|| -> Result<_, SyntaxError> {#fn_body}), parser: RefCell::new(Rc::clone(self)) }
        }
    };

    TokenStream::from(expanded)
}
