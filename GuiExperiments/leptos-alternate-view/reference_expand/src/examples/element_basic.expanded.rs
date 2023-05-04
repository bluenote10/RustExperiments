mod element_basic {
    use leptos::*;
    fn main() {
        mount_to_body(|cx| {
            leptos::leptos_dom::html::div(cx)
                .child((cx, #[allow(unused_braces)] "Hello world"))
                .with_view_marker("reference_expand-src-examples-element_basic.rs-5")
        })
    }
}
