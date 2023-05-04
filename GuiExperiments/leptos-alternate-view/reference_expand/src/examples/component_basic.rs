use leptos::*;

#[component]
fn Main(cx: Scope, some_int: i32) -> impl IntoView {
    view! {
        cx,
        <div>
            "Hello world"
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <Main some_int=42/>
        }
    })
}
