#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn BasicSlider() -> Element {
    let mut value = use_signal(|| 50_i32);
    let mut frozen = use_signal(|| false);

    use_effect(move || log::info!("Count changed to {value}"));

    rsx! {
        div {
            div { "Value: {value}" }
            div {
                input {
                    r#type: "range",
                    value: "{value}",
                    oninput: move |event| {
                        if !frozen() {
                            value.set(event.value().parse::<i32>().unwrap());
                        }
                    }
                }
            }
            div {
                "Frozen"
                input {
                    r#type: "checkbox",
                    value: "{frozen}",
                    oninput: move |event| {
                        frozen.set(event.value().parse::<bool>().unwrap());
                    }
                }
            }
            button { onclick: move |_event| value.set(50), "Reset" }
        }
    }
}
