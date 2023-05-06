extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;

use crate::expressions::Expressions;

pub fn fragment(input: TokenStream) -> TokenStream {
    match syn::parse2::<Expressions>(input) {
        Ok(expressions) => transform(expressions),
        Err(err) => err.into_compile_error(),
    }
}

fn transform(expressions: Expressions) -> TokenStream {
    let expressions = expressions.0;
    let output = quote! {
        ::leptos::Fragment::lazy(|| <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                #(#expressions),*
            ]),
        ))
    };
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::compare;
    use pretty_assertions::assert_eq;
    use syn::Result;

    fn parse(input: TokenStream) -> Result<Expressions> {
        syn::parse2::<Expressions>(input)
    }

    #[test]
    fn test_parse() {
        let input = quote!();
        let result = parse(input).unwrap();
        assert_eq!(result.0.len(), 0);

        let input = quote!(foo);
        let result = parse(input).unwrap();
        assert_eq!(result.0.len(), 1);

        let input = quote!(foo,);
        let result = parse(input).unwrap();
        assert_eq!(result.0.len(), 1);

        let input = quote!(foo, bar);
        let result = parse(input).unwrap();
        assert_eq!(result.0.len(), 2);

        let input = quote!(foo, bar,);
        let result = parse(input).unwrap();
        assert_eq!(result.0.len(), 2);
    }

    #[test]
    fn test_fragment_single_child() {
        let input = quote!(child_a);
        let output = fragment(input);
        let output_expected = quote! {
            ::leptos::Fragment::lazy(|| <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([child_a]),
            ))
        };
        compare!(output, output_expected);
    }

    #[test]
    fn test_fragment_multiple_children() {
        let input = quote!(child_a, child_b);
        let output = fragment(input);
        let output_expected = quote! {
            ::leptos::Fragment::lazy(|| <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([child_a, child_b]),
            ))
        };
        compare!(output, output_expected);
    }
}
