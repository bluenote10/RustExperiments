use proc_macro2::TokenStream;
use quote::quote;

macro_rules! compare {
    ($output:expr, $output_expected:expr) => {
        let output_pretty = crate::test_utils::prettify($output);
        let output_expected_pretty = crate::test_utils::prettify($output_expected);
        println!(
            "Output:\n{}\nOutput expected\n:{}",
            output_pretty, output_expected_pretty
        );
        pretty_assertions::assert_eq!(output_pretty, output_expected_pretty);
    };
}

pub(crate) use compare;

// https://stackoverflow.com/a/74360109/1804173
pub fn prettify(stream: TokenStream) -> String {
    let wrapped = quote!(
        fn wrapped() { #stream }
    );
    match syn::parse_file(&wrapped.to_string()) {
        Ok(file) => prettyplease::unparse(&file),
        Err(err) => {
            panic!(
                "Failed to parse token stream: {}; input was:\n{}",
                err,
                wrapped.to_string()
            );
        }
    }
}
