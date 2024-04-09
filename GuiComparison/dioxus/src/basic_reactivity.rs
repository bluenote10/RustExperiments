#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn ReactiveParent() -> Element {
    let mut w = use_signal(|| 50_i32);
    let mut h = use_signal(|| 50_i32);

    rsx! {
        div {
            div {
                input {
                    r#type: "range",
                    value: "{w}",
                    oninput: move |event| {
                        w.set(event.value().parse::<i32>().unwrap());
                    }
                }
            }
            div {
                input {
                    r#type: "range",
                    value: "{h}",
                    oninput: move |event| {
                        h.set(event.value().parse::<i32>().unwrap());
                    }
                }
            }
            ReactiveChild { w: w(), h: h() }
        }
    }
}

#[component]
pub fn ReactiveChild(w: i32, h: i32) -> Element {
    rsx! { div { width: "{w}px", height: "{h}px", border: "1px solid #AAA", "{w} x {h}" } }
}
