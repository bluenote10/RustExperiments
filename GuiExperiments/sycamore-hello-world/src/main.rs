use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, World!" }
            p { "Another paragraph." }
            div {
                "This is a div"
                div { "nested" }
            }
            Counter
            TimerCounter {}
        }
    });
}

#[component]
fn Counter<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = |_| state.set(*state.get() + 1);
    let decrement = |_| state.set(*state.get() - 1);
    let reset = |_| state.set(0);
    view! { cx,
        div {
            p { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }
        }
    }
}

#[component]
fn TimerCounter<G: Html>(cx: Scope) -> View<G> {
    let state = create_signal(cx, 0);

    spawn_local_scoped(cx, async move {
        loop {
            TimeoutFuture::new(1000).await;
            state.set(*state.get() + 1);
        }
    });

    view! { cx,
        div {
            p { "Value: " (state.get()) }
        }
    }
}
