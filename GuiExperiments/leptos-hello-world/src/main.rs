mod style;

use lazy_static::lazy_static;
use leptos::*;
use style::{create_style, css};

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(cx, initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        cx,
        <div class=&*STYLE>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}

// const STYLE: String = create_style();

// lazy_static! {
//     static ref STYLE: String = create_style("color: #00F;");
// }

// fn style() -> &'static str {
//     &*STYLE
// }

css!(
    STYLE,
    r#"
    color: #0F0;
    font-size: 20px;

    & button {
        padding: 10px;
        color: #F00;
        font-size: 30px;
    }
    "#
);

css!(
    STYLE2,
    r#"
    color: #0F0;

    & button {
        padding: 10px;
        color: #F00;
    }
    "#
);

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    log!("Mounting to body...");
    // add_css().expect("Failed to add CSS");
    mount_to_body(|cx| view! { cx,  <SimpleCounter initial_value=3 /> })
}
