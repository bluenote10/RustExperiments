use leptos::*;
use leptos_alternate_macro::c;

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

/*
fn leptos_test() {
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
*/

fn main() {
    //comp!((foo, bar));
    //comp!(foo);

    //comp!(foo, bar);
    //comp! {foo, bar};

    let style = "foo";

    comp![div { class: "foo" }(
        comp![p("Hello")],
        comp![p("World")],
        comp![button { on_click: |_| {} }("World")],
    )];

    comp!(-div { class: "foo" }(
        -p("Hello"),
        -p("Hello"),
        -p("World"),
        -button { on_click: |_| {} }("World"),
    ));

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

    log!("Mounting to body...");

    mount_to_body(|cx| {
        let style = "style";
        c![div(
            // children
            "Hello World",
            c![p("paragraph")],
            c![p("paragraph")],
        )]
    });
}
