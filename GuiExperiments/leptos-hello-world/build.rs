use glob::glob;
use std::{env::current_dir, fs};
use syn::ext::IdentExt;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Item;
use syn::{parse_quote, Expr, ExprLit, Ident, Lit, LitStr, Macro, Token};

// Getting debug output from a build.rs is not easy. Using a warning as a work-around,
// but that only allows single line output.
// https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
// For full output use:
// less ./target/debug/build/leptos-hello-world-*/output
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    p!("Running build script...");
    p!("Working dir: {:?}", current_dir().unwrap());

    let pattern = format!("{}/src/**/*.rs", current_dir().unwrap().to_str().unwrap());
    for file in glob(&pattern).unwrap() {
        let file = file.unwrap();
        p!("{:?}", file);

        let content = fs::read_to_string(file).expect("Failed to read file");
        p!("{}", content);

        let ast = syn::parse_file(&content).unwrap();
        // p!("{:#?}", ast);

        for item in ast.items {
            if let Item::Macro(item_macro) = item {
                let path = &item_macro.mac.path;
                if path.leading_colon.is_none()
                    && path.segments.len() == 1
                    && path.segments[0].ident == "css"
                {
                    p!("Found css macro: {:#?}", item_macro);
                    //item_macro.mac.tokens.
                    let body = item_macro.mac.parse_body::<CssMacroCall>();
                    p!("Body: {:?}", body);
                }
            }
        }

        // let re = regex::Regex::new(r"css!\((.*)\)").unwrap();
        // let cap = re.captures(&content);
        // p!("{:#?}", cap);
    }
}

#[derive(Debug)]
struct CssMacroCall {
    css_var_name: String,
    css_content: String,
}

impl Parse for CssMacroCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let css_var_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let css_content: LitStr = input.parse()?;

        Ok(CssMacroCall {
            css_var_name: css_var_name.to_string(),
            css_content: css_content.value(),
        })
    }
}
