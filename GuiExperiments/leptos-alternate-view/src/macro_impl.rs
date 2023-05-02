extern crate proc_macro;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Error, Expr, FieldValue, Ident, LitStr, Path, Result, Token,
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

#[derive(Debug)]
struct CompExpr {
    ident: Ident,
    fields: Vec<FieldValue>,
    children: Vec<Expr>,
}

impl Parse for CompExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr: Expr = input.parse()?;

        let (expr_struct, children) = if let Expr::Call(expr_call) = expr {
            // convert from Punctuated<Expr, Comma> to Vec<_>
            let args: Vec<_> = expr_call.args.iter().map(|arg| arg.clone()).collect();
            (*expr_call.func.clone(), args)
        } else {
            (expr, vec![])
        };

        match &expr_struct {
            Expr::Struct(expr_struct) => {
                let Some(ident) = expr_struct.path.get_ident() else {
                    return Err(Error::new_spanned(expr_struct.path.clone(), "A plain identifier is required"))
                };
                // convert from Punctuated<FieldValue, Comma> to Vec<_>
                let fields: Vec<_> = expr_struct.fields.iter().map(|fv| fv.clone()).collect();
                Ok(CompExpr {
                    ident: ident.clone(),
                    fields,
                    children,
                })
            }
            _ => Err(Error::new_spanned(expr_struct, "Unsupported expression")),
        }
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
        let comp_expr = parse_comp_expr(stream).unwrap();
        assert_eq!(comp_expr.ident.to_string(), "SomeComponent");
        assert_eq!(comp_expr.fields.len(), 1);
        assert_eq!(comp_expr.children.len(), 0);

        let stream = quote!(SomeComponent { prop_a: value }(child));
        let comp_expr = parse_comp_expr(stream).unwrap();
        assert_eq!(comp_expr.ident.to_string(), "SomeComponent");
        assert_eq!(comp_expr.fields.len(), 1);
        assert_eq!(comp_expr.children.len(), 1);

        let stream = quote!(SomeComponent { prop_a: value }(child_a, child_b));
        let comp_expr = parse_comp_expr(stream).unwrap();
        assert_eq!(comp_expr.ident.to_string(), "SomeComponent");
        assert_eq!(comp_expr.fields.len(), 1);
        assert_eq!(comp_expr.children.len(), 2);
    }
}
