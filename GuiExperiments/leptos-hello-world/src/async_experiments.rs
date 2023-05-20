use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn load_data() -> i32 {
    TimeoutFuture::new(1_000).await;
    42
}

#[component]
fn Child(cx: Scope, #[prop(into)] is_odd: Signal<bool>) -> impl IntoView {
    view! { cx,
        <p>
            {if is_odd.get() { "odd" } else { "even" }}
        </p>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let async_data = create_resource(cx, || {}, |_| async move { load_data().await });

    let derived_is_odd = move || async_data.read(cx).map(|value| value % 2 == 1);

    view! { cx,
        {match derived_is_odd() {
            Some(is_odd) => view! { cx, <Child is_odd=Signal::derive(cx, move || is_odd) /> }.into_view(cx),
            _ => view! { cx, <p>"loading..."</p> }.into_view(cx),
        }}

    }
}
