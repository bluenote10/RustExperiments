use crate::web::canvas::{CanvasWrapper, CanvasWrapperProps};
use leptos::*;

#[component]
fn Main(cx: Scope) -> impl IntoView {
    let (visible, set_visible) = create_signal(cx, true);

    let toggle = move |_| set_visible.update(|v| *v = !*v);

    view! {
        cx,
        <>
            <button on:click=toggle>"Toggle"</button>
            <Show when=visible fallback=|_| ()>
                <CanvasWrapper />
            </Show>
        </>
    }
}

pub fn run_app() {
    console_error_panic_hook::set_once();
    log!("Mounting to body...");

    mount_to_body(|cx| {
        view! {
            cx,
            <Main />
        }
    })
}
