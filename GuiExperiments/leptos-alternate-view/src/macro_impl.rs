extern crate proc_macro;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Expr, Ident, LitStr, Result, Token,
};

pub fn comp(input: TokenStream) -> TokenStream {
    println!("raw input:\n{:#?}", input);

    // let input: TokenTree = Group::new(Delimiter::Parenthesis, input).into();
    // let input: TokenStream = input.into();
    // println!("wrapped input:\n{:#?}", input);

    match parse_input(input) {
        Ok(comp_body) => {
            println!("{:#?}", comp_body);

            quote! {
                log!("Hello world from macro, using parent scope symbol: {}", style);
            }
            .into()
        }
        Err(err) => {
            //quote_spanned!(err.span() => compile_error!("Could not parse token stream as expression")).into()
            err.into_compile_error()
        }
    }

    /*
    let input_span = input.span();
    let Ok(s) = syn::parse2::<Expr>(input) else {
        return quote_spanned!(input_span => compile_error!("Could not parse token stream as expression")).into()
    };
    */
}

fn parse_input(input: TokenStream) -> Result<CompBody> {
    syn::parse2::<CompBody>(input)
}

#[derive(Debug)]
struct CompBody {
    expressions: Vec<Expr>,
}

impl Parse for CompBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let expressions: Vec<_> = input
            .parse_terminated(Expr::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(CompBody { expressions })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let stream: TokenStream = quote!();
        let result = parse_input(stream).unwrap();
        assert_eq!(result.expressions.len(), 0);

        let stream: TokenStream = quote!(foo);
        let result = parse_input(stream).unwrap();
        assert_eq!(result.expressions.len(), 1);

        let stream: TokenStream = quote!(foo,);
        let result = parse_input(stream).unwrap();
        assert_eq!(result.expressions.len(), 1);

        let stream: TokenStream = quote!(foo, bar);
        let result = parse_input(stream).unwrap();
        assert_eq!(result.expressions.len(), 2);

        let stream: TokenStream = quote!(foo, bar,);
        let result = parse_input(stream).unwrap();
        assert_eq!(result.expressions.len(), 2);
    }
}
