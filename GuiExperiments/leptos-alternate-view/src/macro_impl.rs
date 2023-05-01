extern crate proc_macro;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Expr};

pub fn comp(input: TokenStream) -> TokenStream {
    println!("raw input:\n{:#?}", input);

    let input: TokenTree = Group::new(Delimiter::Parenthesis, input).into();
    let input: TokenStream = input.into();

    println!("wrapped input:\n{:#?}", input);

    let input_span = input.span();
    let Ok(s) = syn::parse2::<Expr>(input) else {
        return quote_spanned!(input_span => compile_error!("Could not parse token stream as expression")).into()
    };

    println!("{:#?}", s);

    quote! {
        log!("Hello world from macro, using parent scope symbol: {}", style);
    }
    .into()
}
