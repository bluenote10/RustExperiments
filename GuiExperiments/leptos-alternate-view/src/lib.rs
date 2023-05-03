mod macro_impl;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = macro_impl::comp(input);

    proc_macro::TokenStream::from(output)
}

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = macro_impl::macro_c(input);

    proc_macro::TokenStream::from(output)
}
