use leptos::*;

#[component]
fn TakesChildren(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div>
            {children(cx)}
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <TakesChildren>
                <p>"first"</p>
                <p>"second"</p>
            </TakesChildren>
        }
    })
}
