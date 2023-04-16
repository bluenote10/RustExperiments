use leptos::*;
use stylers::style;

const PARENT_STYLE: &str = style! {"Parent",
    p {
        color: darkblue;
    }

    div {
        border: 1px solid darkblue;
    }

    ul {
        margin: 16px;
    }

    li {
        border: 1px solid green;
    }

    .selected {
        background-color: lightblue;
    }
};

#[component]
pub fn Parent(cx: Scope) -> impl IntoView {
    view! {
        cx,
        class = PARENT_STYLE,
        <>
            <p>"This is a paragraph in the parent."</p>
            <div>"This is a div in the parent."</div>

            <ul>
                <li>"Item 1"</li>
                <li class="selected">"Item 2"</li>
                <li>"Item 3"</li>
            </ul>

            <Child>
                <p>"This paragraph comes from the parent, and accordingly is styled according to parent styles."</p>
            </Child>

            <Child>
                <p>"This paragraph comes from the parent, and accordingly is styled according to parent styles."</p>
            </Child>
        </>
    }
}

#[component]
pub fn Child(cx: Scope, children: Children) -> impl IntoView {
    let styler_class = style! {"Child",
        p {
            color: seagreen;
        }

        div {
            border: 1px solid seagreen;
        }

        .selected {
            background-color: lightgreen;
        }
    };

    let (selected, set_selected) = create_signal(cx, false);

    view! {
        cx,
        class = styler_class,
        // This causes a bug: <div class=move || if selected() { "selected".to_string() } else { "".to_string() }>
        <div class:selected={selected}>
            <p>"This is a paragraph in the child."</p>

            {children(cx)}

            <p>"Selected: " {selected}</p>

            <button on:click=move |_| set_selected.update(|v| *v = !*v)>"Toggle"</button>
        </div>
    }
}
