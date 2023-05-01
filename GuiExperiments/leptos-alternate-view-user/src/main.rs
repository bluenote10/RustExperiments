use leptos::*;
use leptos_alternate_view::comp;

/*
#[component]
fn Main(cx: Scope) -> impl IntoView {
    comp!(foo);

    view! {
        cx,
        <div>
            "Hello world"
        </div>
    }
}

fn main() {
    log!("Mounting to body...");

    mount_to_body(|cx| {
        view! {
            cx,
            <Main />
        }
    })
}
*/

fn main() {
    //comp!((foo, bar));
    //comp!(foo);

    //comp!(foo, bar);
    //comp! {foo, bar};

    let style = "foo";

    comp!(
        h("text"),
        r#div {}("text"),
        ComponentWithoutChildren { name: "foo" },
        ComponentWithChildren { name: "foo" }(span("hello"), span("world")),
        div {
            on_error: || {
                asjklfaklsdhfklasjdhflksajdhfklajsdhflkjsahdklfjhaskldjfhaklsjfdhlaksjdhflkajsfdh
            },
        }(
            div {
                on_error: || {
                    asjklfaklsdhfklasjdhflksajdhfklajsdhflkjsahdklfjhaskldjfhaklsjfdhlaksjdhflkajsfdh
                },
            },
            div {
                on_error: || {
                    asjklfaklsdhfklasjdhflksajdhfklajsdhflkjsahdklfjhaskldjfhaklsjfdhlaksjdhflkajsfdh
                },
            },
        ),
        div {
            on_error: || {
                asjklfaklsdhfklasjdhflksajdhfklajsdhflkjsahdklfjhaskldjfhaklsjfdhlaksjdhflkajsfdh
            },
        },
        div {
            on_error: || {
                asjklfaklsdhfklasjdhflksajdhfklajsdhflkjsahdklfjhaskldjfhaklsjfdhlaksjdhflkajsfdh
            },
        }
    );
}
