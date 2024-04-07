#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn LockedSliders() -> Element {
    let mut locked = use_signal(|| false);
    let mut value1 = use_signal(|| 25_i32);
    let mut value2 = use_signal(|| 75_i32);

    use_effect(move || log::info!("Value 1 changed: {value1}"));
    use_effect(move || log::info!("Value 2 changed: {value2}"));
    use_effect(move || log::info!("Either value changed: {value1} / {value2}"));

    rsx! {
        div {
            div { "Value 1: {value1}, Value 2: {value2}, Locked: {locked}" }
            div {
                input {
                    r#type: "range",
                    value: "{value1}",
                    oninput: move |event| {
                        value1.set(event.value().parse::<i32>().unwrap());
                    }
                }
            }
            div {
                input {
                    r#type: "range",
                    value: "{value2}",
                    oninput: move |event| {
                        value2.set(event.value().parse::<i32>().unwrap());
                    }
                }
            }
            div {
                "Locked"
                input {
                    r#type: "checkbox",
                    value: "{locked}",
                    oninput: move |event| {
                        log::info!("Event: {event:?}");
                        locked.set(event.value().parse::<bool>().unwrap());
                    }
                }
            }
            button { onclick: move |_event| value1.set(50), "Reset" }
        }
    }
}
