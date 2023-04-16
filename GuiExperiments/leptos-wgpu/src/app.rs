use crate::canvas::{CanvasWrapper, CanvasWrapperProps};
use leptos::*;

pub fn run_app() {
    log!("Mounting to body...");
    mount_to_body(|cx| {
        view! {
            cx,
            <CanvasWrapper />
        }
    })
}
