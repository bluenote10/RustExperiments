#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::utils::console_log;

mod utils;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    console_log!("Hello {}!", "world");
    console_log!("2 + 4 = {}", 2 + 4);

    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}
