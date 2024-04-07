#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn BasicSlider() -> Element {
    let mut value = use_signal(|| 50_i32);
    use_effect(move || log::info!("Count changed to {value}"));

    rsx! {
        div {
            div { "Value: {value}" }
            div {
                input {
                    r#type: "range",
                    value: "{value}",
                    oninput: move |event| {
                        log::info!("Event: {event:?}");
                        value.set(event.value().parse::<i32>().unwrap());
                    }
                }
            }
            button { onclick: move |_event| value.set(50), "Reset" }
        }
    }
}
