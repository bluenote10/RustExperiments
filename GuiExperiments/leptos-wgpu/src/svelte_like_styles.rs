use leptos::*;

#[component]
pub fn Parent(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <>
            <p class=STYLE_PARENT>"This is a paragraph in the parent."</p>
            <div class=STYLE_PARENT>"This is a div in the parent."</div>

            <ul class=STYLE_PARENT>
                <li class=STYLE_PARENT>"Item 1"</li>
                <li class={format!("{} selected", STYLE_PARENT)}>"Item 2"</li>
                <li class=STYLE_PARENT>"Item 3"</li>
            </ul>

            <Child>
                <p class=STYLE_PARENT>"This paragraph comes from the parent, and accordingly is styled according to parent styles."</p>
            </Child>

            <Child>
                <p class=STYLE_PARENT>"This paragraph comes from the parent, and accordingly is styled according to parent styles."</p>
            </Child>
        </>
    }
}

#[component]
pub fn Child(cx: Scope, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, false);

    view! {
        cx,
        <div class=move || if selected() { format!("{} selected", STYLE_CHILD) } else { STYLE_CHILD.into() }>
            <p class=STYLE_CHILD>"This is a paragraph in the child."</p>

            {children(cx)}

            <p class=STYLE_CHILD>"Selected: " {selected}</p>

            <button class=STYLE_CHILD on:click=move |_| set_selected.update(|v| *v = !*v)>"Toggle"</button>
        </div>
    }
}

// Let's ignore where these CSS names actually come from. That's a separate problem.
const STYLE_PARENT: &str = "s-azXUKLSuEAYX";
const STYLE_CHILD: &str = "s-fVHJuYNM6LRl";

/*
#[component(css_scope_class=STYLE_PARENT)]
pub fn Parent(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <>
            <p>"This is a paragraph in the parent."</p>
            <div>"This is a div in the parent."</div>

            <ul>
                <li>"Item 1"</li>
                <li class:selected>"Item 2"</li>
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

#[component(css_scope_class=STYLE_CHILD)]
pub fn Child(cx: Scope, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(cx, false);

    view! {
        cx,
        <div class:selected>
            <p>"This is a paragraph in the child."</p>

            {children(cx)}

            <p>"Selected: " {selected}</p>

            <button on:click=move |_| set_selected.update(|v| *v = !*v)>"Toggle"</button>
        </div>
    }
}
*/
