extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, FieldValue, Ident, Member, Result, Token,
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

pub fn macro_c(input: TokenStream) -> TokenStream {
    match syn::parse2::<CompExpr>(input) {
        Ok(comp_expr) => transform_comp(comp_expr),
        Err(err) => {
            //quote_spanned!(err.span() => compile_error!("Could not parse token stream as expression")).into()
            err.into_compile_error()
        }
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

        // Unwrap call expression if it is one
        let (expr, children) = if let Expr::Call(expr_call) = expr {
            (*expr_call.func.clone(), flatten_punctuated(&expr_call.args))
        } else {
            (expr, vec![])
        };

        match &expr {
            Expr::Struct(expr_struct) => {
                let Some(ident) = expr_struct.path.get_ident() else {
                    return Err(Error::new_spanned(expr_struct.path.clone(), "A plain identifier is required"))
                };
                Ok(CompExpr {
                    ident: ident.clone(),
                    fields: flatten_punctuated(&expr_struct.fields),
                    children,
                })
            }
            Expr::Path(expr_path) => {
                let Some(ident) = expr_path.path.get_ident() else {
                    return Err(Error::new_spanned(expr_path.clone(), "A plain identifier is required"))
                };
                Ok(CompExpr {
                    ident: ident.clone(),
                    fields: vec![],
                    children,
                })
            }
            _ => Err(Error::new_spanned(expr, "Unsupported expression")),
        }
    }
}

fn transform_comp(comp_expr: CompExpr) -> TokenStream {
    let ident = comp_expr.ident;
    let is_element = ident.to_string().chars().next().unwrap().is_lowercase();

    if is_element {
        let mut expr: TokenStream = quote!(::leptos::leptos_dom::html::#ident(cx));

        for child in comp_expr.children {
            // Leptos seems to generate an `#[allow(unused_braces)]` before the child expressions,
            // but I'm not sure if this is needed when using the alternate macro.
            // expr = quote!(#expr.child((cx, #[allow(unused_braces)] #child)))
            expr = quote!(#expr.child((cx, #child)))
        }

        // TODO: Understand why the view macro sets `class=...` via `.attr("class", ...)` and not directly
        // via `.class`. How do multiple class values behave?
        expr = quote!(#expr.attr("class", (cx, style)));

        for field in comp_expr.fields {
            let Member::Named(ident) = field.member else { continue };
            let ident_str = ident.to_string();
            let value = field.expr;
            if ident_str == "on_mount" {
                expr = quote!(#expr.on_mount(#value))
            } else if ident_str.starts_with("on_") {
                let event_ident = Ident::new(&format!("{}Props", &ident_str[3..]), ident.span());
                expr = quote!(#expr.on(::leptos::ev::#event_ident, #value))
            } else {
                expr = quote!(#expr.attr(#ident_str, (cx, #value)))
            }
        }

        expr
    } else {
        let props = Ident::new(&format!("{}Props", ident.to_string()), ident.span());
        let mut prop_builder_expr: TokenStream = quote!( #props::builder() );

        for field in comp_expr.fields {
            let Member::Named(ident) = field.member else { continue };
            let prop = ident;
            let value = field.expr;
            prop_builder_expr = quote!(#prop_builder_expr.#prop(#value))
        }
        prop_builder_expr = quote!(#prop_builder_expr.build());

        /*
        for child in comp_expr.children {
            expr = quote!(#expr.child((cx, #[allow(unused_braces)] #child))).into()
        }
        */

        quote!( #ident(cx, #prop_builder_expr)).into()
    }
}

fn flatten_punctuated<T, P>(punctuated: &Punctuated<T, P>) -> Vec<T>
where
    T: Clone,
{
    punctuated.iter().map(|arg| arg.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::compare;
    use pretty_assertions::assert_eq;

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
        let stream = quote!(div(child));
        let comp_expr = parse_comp_expr(stream).unwrap();
        assert_eq!(comp_expr.ident.to_string(), "div");
        assert_eq!(comp_expr.fields.len(), 0);
        assert_eq!(comp_expr.children.len(), 1);

        let stream = quote!(div(child_a, child_b));
        let comp_expr = parse_comp_expr(stream).unwrap();
        assert_eq!(comp_expr.ident.to_string(), "div");
        assert_eq!(comp_expr.fields.len(), 0);
        assert_eq!(comp_expr.children.len(), 2);

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

    #[test]
    fn test_macro_element_basic() {
        let input = quote!(div(child));
        let output = macro_c(input);
        let output_expected = quote! {
            ::leptos::leptos_dom::html::div(cx)
                .child((cx, child))
                .attr("class", (cx, style))
        };
        compare!(output, output_expected);
    }

    #[test]
    fn test_macro_element_with_event_handler() {
        let input = quote!(button { on_click: |_| {} }("Click me"));
        let output = macro_c(input);
        let output_expected = quote! {
            ::leptos::leptos_dom::html::button(cx)
                .child((cx, "Click me"))
                .attr("class", (cx, style))
                .on(::leptos::ev::clickProps, |_| {})
        };
        compare!(output, output_expected);
    }

    #[test]
    fn test_macro_element_with_on_mount_handler() {
        let input = quote!(div { on_mount: |_| {} });
        let output = macro_c(input);
        let output_expected = quote! {
            ::leptos::leptos_dom::html::div(cx)
                .attr("class", (cx, style))
                .on_mount(|_| {})
        };
        compare!(output, output_expected);
    }

    #[test]
    fn test_macro_component_basic() {
        let input = quote!(Main { some_int: 42 });
        let output = macro_c(input);
        let output_expected = quote! {
            Main(cx, MainProps::builder().some_int(42).build())
        };
        compare!(output, output_expected);
    }
}
