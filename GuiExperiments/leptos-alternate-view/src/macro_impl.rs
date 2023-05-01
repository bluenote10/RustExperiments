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

    match syn::parse2::<CompBody>(input) {
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

#[derive(Debug)]
struct CompBody {
    expressions: Vec<CompExpr>,
}

impl Parse for CompBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let expressions: Vec<_> = input
            .parse_terminated(CompExpr::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(CompBody { expressions })
    }
}

#[derive(Debug)]
struct CompExpr {
    expr: Expr,
}

impl Parse for CompExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr: Expr = input.parse()?;

        match &expr {
            Expr::Struct(expr_struct) => (),
            _ => (),
        }

        Ok(CompExpr { expr })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_comp_body(input: TokenStream) -> Result<CompBody> {
        syn::parse2::<CompBody>(input)
    }

    #[test]
    fn test_parse_comp_body() {
        let stream = quote!();
        let result = parse_comp_body(stream).unwrap();
        assert_eq!(result.expressions.len(), 0);

        let stream = quote!(foo);
        let result = parse_comp_body(stream).unwrap();
        assert_eq!(result.expressions.len(), 1);

        let stream = quote!(foo,);
        let result = parse_comp_body(stream).unwrap();
        assert_eq!(result.expressions.len(), 1);

        let stream = quote!(foo, bar);
        let result = parse_comp_body(stream).unwrap();
        assert_eq!(result.expressions.len(), 2);

        let stream = quote!(foo, bar,);
        let result = parse_comp_body(stream).unwrap();
        assert_eq!(result.expressions.len(), 2);
    }

    fn parse_comp_expr(input: TokenStream) -> Result<CompExpr> {
        syn::parse2::<CompExpr>(input)
    }

    #[test]
    fn test_parse_comp_expr() {
        let stream = quote!(SomeComponent { prop_a: value });
        let result = parse_comp_expr(stream).unwrap();
        //assert_eq!(result.expressions.len(), 1);
    }
}
