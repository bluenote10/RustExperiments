#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::utils::console_log;

mod utils;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    // Note working like that?
    // let value: &mut u32 = cx.use_hook(|| 0);
    let mut value = use_state(cx, || 0);

    use_effect(cx, (value,), |(value,)| async move {
        console_log!("Running effect, value changed to: {}", value);
    });

    console_log!("Re-rendering, value: {}", value);

    cx.render(rsx! {
        div {
            "Hello, world!"
        },
        div {
            "Counter: {value}"
        },
        button { onclick: move |_| value += 1, "Increment" }
    })
}
