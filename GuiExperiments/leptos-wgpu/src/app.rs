use crate::components::{Parent, ParentProps};
use leptos::*;

pub fn run_app() {
    log!("Mounting to body...");
    mount_to_body(|cx| {
        view! {
            cx,
            <Parent />
        }
    })
}
