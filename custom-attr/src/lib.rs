#![feature(proc_macro_quote)]
extern crate proc_macro;

use proc_macro::{TokenStream, quote};

#[proc_macro_attribute]
pub fn add_no_std(_attr_tokens: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut prelude_tokens = quote!{ #![no_std] };
    prelude_tokens.extend(tokens);
    prelude_tokens
}
