mod fragment_basic {
    use leptos::*;
    fn main() {
        mount_to_body(|cx| {
            {
                leptos::Fragment::lazy(|| <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            leptos::leptos_dom::html::div(cx).into_view(cx),
                            leptos::leptos_dom::html::div(cx).into_view(cx),
                            leptos::leptos_dom::html::div(cx).into_view(cx),
                        ]),
                    ))
                    .with_view_marker(
                        "reference_expand-src-examples-fragment_basic.rs-5",
                    )
            }
        })
    }
}
