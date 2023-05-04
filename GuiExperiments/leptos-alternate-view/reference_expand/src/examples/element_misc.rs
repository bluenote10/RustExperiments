use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <div class="foo">
                <p>"Hello"</p>
                <p>"World"</p>
                <button on:click=|_| {}>"Click me"</button>
            </div>
        }
    })
}
