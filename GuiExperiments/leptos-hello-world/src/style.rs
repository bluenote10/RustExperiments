use lazy_static::lazy_static;
use leptos::log;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::Element;

///
/// Prior art svelte:
/// - https://github.com/sveltejs/svelte/issues/583
/// - https://github.com/sveltejs/svelte/issues/1118
///
/// CSS nesting:
/// - https://blog.logrocket.com/native-css-nesting/
/// - https://kilianvalkhof.com/2021/css-html/css-nesting-specificity-and-you/
///
/// Prior art css-in-rust:
/// - https://github.com/lukidoescode/css-in-rust/blob/master/Cargo.toml
/// - https://github.com/lukidoescode/css-in-rust/blob/master/src/style/mod.rs
///

fn generate_element(class_name: &str, css: &str) -> Result<Element, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let style_element = document.create_element("style").unwrap();
    // style_element.set_text_content(Some("div.mystyle428938 { color: #F00; }"));
    style_element.set_text_content(Some(&format!(".{} {{ {} }}", class_name, css)));
    Ok(style_element)
}

pub fn add_css(class_name: &str, css: &str) -> Result<(), JsValue> {
    let node = generate_element(class_name, css)?;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let head = document.head().expect("should have a head in document");
    head.append_child(&node).ok();
    Ok(())
}

lazy_static! {
    static ref STYLE_NAMES: HashSet<String> = HashSet::new();
}
static mut STYLE_COUNT: i32 = 0;

pub fn create_style(css: &'static str) -> String {
    unsafe {
        let style_name = format!("style-rs-{}", STYLE_COUNT);
        STYLE_COUNT += 1;
        add_css(&style_name, css).expect("Failed to create style");
        log!("Created class '{}' with content {}", style_name, css);
        style_name
    }
}

macro_rules! css {
    ($var_name:ident, $css:expr) => {
        lazy_static! {
            static ref $var_name: String = create_style($css);
        }
    };
}
pub(crate) use css;

// Offering an accessor function would have been nice due to lower casing
// and avoiding the ugliness of &*STYLE. But it is most likely not possible,
// because macro hygiene does not include statics, and generating a unique
// internal identifier is probably also not possible.
// https://stackoverflow.com/questions/36240846/why-does-macro-hygiene-not-prevent-collisions-between-multiple-const-definitions
// macro_rules! css {
//     ($var_name:ident, $css:expr) => {
//         lazy_static! {
//             static ref _$var_name: String = create_style($css);
//         }
//         fn $var_name() -> &'static str {
//             &*STYLE
//         }
//     };
// }
// pub(crate) use css;
