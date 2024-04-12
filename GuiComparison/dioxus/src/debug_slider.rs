#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn DebugSlider() -> Element {
    let mut value = use_signal(|| 0.0_f32);
    let mut only_positive = use_signal(|| false);

    rsx! {
        div {
            div { "value: {value}" }
            div { "only_positive: {only_positive}" }
            div {
                input {
                    r#type: "range",
                    value: "{value}",
                    min: "-1.0",
                    max: "1.0",
                    step: "0.01",
                    prevent_default: "oninput",
                    oninput: move |event| {
                        let mut incoming_value = event.value().parse::<f32>().unwrap();
                        if only_positive() {
                            incoming_value = f32::max(0.0, incoming_value);
                        }
                        value.set(incoming_value);
                    }
                }
            }
            div {
                "Only positive"
                input {
                    r#type: "checkbox",
                    value: "{only_positive}",
                    oninput: move |event| {
                        only_positive.set(event.value().parse::<bool>().unwrap());
                    }
                }
            }
        }
    }
}
