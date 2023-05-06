use leptos::*;
use leptos_alternate_macro::{c, fragment};

#[component]
fn Basic(cx: Scope) -> impl IntoView {
    let style = "some-class";

    c![div(
        // children
        "Hello World",
        c![p("paragraph")],
        c![br],
        c![p("paragraph")],
    )]
}

#[component]
fn WithFragments(cx: Scope) -> impl IntoView {
    let style = "some-class";

    fragment![c!(p("first")), c!(p("second")),]
}

fn main() {
    log!("Mounting to body...");

    mount_to_body(|cx| fragment!(c!(Basic {}), c!(WithFragments {}),));
}
