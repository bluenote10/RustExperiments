mod element_misc {
    use leptos::*;
    fn main() {
        mount_to_body(|cx| {
            leptos::leptos_dom::html::div(cx)
                .attr("class", (cx, "foo"))
                .child((
                    cx,
                    leptos::leptos_dom::html::p(cx)
                        .child((cx, #[allow(unused_braces)] "Hello")),
                ))
                .child((
                    cx,
                    leptos::leptos_dom::html::p(cx)
                        .child((cx, #[allow(unused_braces)] "World")),
                ))
                .child((
                    cx,
                    leptos::leptos_dom::html::button(cx)
                        .on(::leptos::ev::click, |_| {})
                        .child((cx, #[allow(unused_braces)] "Click me")),
                ))
                .with_view_marker("reference_expand-src-examples-element_misc.rs-5")
        })
    }
}
